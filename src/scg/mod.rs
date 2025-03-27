//! System Clock Generator

use crate::{
    pac::scg::Instance,
    port::scg::{Pin, EXTAL48M, XTAL48M},
};

use cfg_if::cfg_if;

mod firc;
pub use firc::FIRC;

cfg_if! {
    if #[cfg(feature = "mcxa2")] {
        mod pll;
        pub use pll::{PllSource, PllConfig};
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SCGError {
    MissingPins,
    Busy,

    InvalidValue,
    InvalidConfig,

    ClockSourceDisabled,

    SOSCErr,
    SIRCErr,
    FIRCErr,
    SPLLErr,
}

pub struct SCG<const N: u8, PINS> {
    scg: Instance<N>,
    pins: PINS,
    has_pins: bool,
    config: SCGConfig,
}
unsafe impl<const N: u8, PINS> Send for SCG<N, PINS> {}
unsafe impl<const N: u8, PINS> Sync for SCG<N, PINS> {}
impl<const N: u8, EXTAL, XTAL> SCG<N, Pins<EXTAL, XTAL>>
where
    EXTAL: Pin<Signal = EXTAL48M>,
    XTAL: Pin<Signal = XTAL48M, Module = EXTAL::Module>,
{
    pub fn new(scg: Instance<N>, mut pins: Pins<EXTAL, XTAL>) -> Self {
        use crate::port::scg::prepare;

        prepare(&mut pins.extal);
        if let Some(xtal) = &mut pins.xtal {
            prepare(xtal);
        }

        Self {
            scg,
            pins,
            has_pins: true,
            config: SCGConfig::default(),
        }
    }

    pub fn release(mut self) -> (EXTAL, Option<XTAL>) {
        self.back_to_default().unwrap();
        (self.pins.extal, self.pins.xtal)
    }

    pub fn pins(extal: EXTAL, xtal: Option<XTAL>) -> Pins<EXTAL, XTAL> {
        Pins { extal, xtal }
    }
}
impl<const N: u8> SCG<N, ()> {
    pub fn without_pins(scg: Instance<N>) -> Self {
        Self {
            scg,
            pins: (),
            has_pins: false,
            config: SCGConfig::default(),
        }
    }
}
impl<const N: u8, PINS> SCG<N, PINS> {
    pub fn freeze(&mut self) -> Result<(), SCGError> {
        if !self.has_pins && self.config.sosc.is_some() {
            // should configure SOSC but no EXTAL or XTAL pin configured.
            return Err(SCGError::MissingPins);
        }
        if !self.config.valid() {
            return Err(SCGError::InvalidConfig);
        }

        let scg = self.scg.regs();

        if let Some(sosc) = self.config.sosc {
            SOSC::enable_sosc(scg, sosc, self.config.sosc_stop_en)?;
        } else {
            SOSC::disable_sosc(scg)?;
        }

        scg.SIRCCSR().modify(|r| r.set_LK(false));
        scg.SIRCCSR().modify(|r| {
            r.set_SIRCSTEN(self.config.sirc_stop_en);
            r.set_SIRC_CLK_PERIPH_EN(self.config.sirc_clk_en);
        });
        scg.SIRCCSR().modify(|r| r.set_LK(true));
        while !scg.SIRCCSR().read().SIRCVLD() {}
        if scg.SIRCCSR().read().SIRCERR() {
            return Err(SCGError::SIRCErr);
        }

        if let Some(firc) = self.config.firc {
            FIRC::enable_firc(
                scg,
                firc,
                self.config.firc_stop_en,
                self.config.firc_fclk_en,
                self.config.firc_sclk_en,
            )?;
        } else {
            FIRC::disable_firc(scg)?;
        }

        #[cfg(feature = "mcxa2")]
        if let Some(spll) = self.config.spll {
            use pll::enable_spll;

            if matches!(spll.source, PllSource::SOSC) && self.config.sosc.is_none() {
                return Err(SCGError::ClockSourceDisabled);
            }
            if matches!(spll.source, PllSource::FIRC)
                && self.config.firc.is_none()
                && !self.config.firc_sclk_en
            {
                return Err(SCGError::ClockSourceDisabled);
            }

            let spll_source_clk_freq = match spll.source {
                PllSource::SOSC => self.config.sosc.unwrap().freq(),
                PllSource::FIRC => FIRC::default().freq(),
                PllSource::ROSC => 32_768,
                PllSource::SIRC => 12_000_000,
            };

            enable_spll(scg, spll, spll_source_clk_freq)?;
        } else {
            // TODO: disable SPLL
        }

        if scg.CSR().read().SCS() == self.config.main_clock_source as u8 {
            return Ok(());
        }
        scg.RCCR()
            .write(|r| r.set_SCS(self.config.main_clock_source as u8));
        while scg.CSR().read().SCS() != self.config.main_clock_source as u8 {}

        Ok(())
    }

    pub fn back_to_default(&mut self) -> Result<(), SCGError> {
        self.config = SCGConfig::default();
        self.freeze()
    }

    pub fn config(&mut self) -> &mut SCGConfig {
        &mut self.config
    }

    pub fn set_config(&mut self, config: SCGConfig) {
        self.config = config;
    }
}

pub struct Pins<EXTAL, XTAL>
where
    EXTAL: Pin<Signal = EXTAL48M>,
    XTAL: Pin<Signal = XTAL48M, Module = EXTAL::Module>,
{
    extal: EXTAL,
    xtal: Option<XTAL>,
}
unsafe impl<EXTAL: Pin<Signal = EXTAL48M>, XTAL: Pin<Signal = XTAL48M, Module = EXTAL::Module>> Send
    for Pins<EXTAL, XTAL>
{
}
unsafe impl<EXTAL: Pin<Signal = EXTAL48M>, XTAL: Pin<Signal = XTAL48M, Module = EXTAL::Module>> Sync
    for Pins<EXTAL, XTAL>
{
}

/// System Oscillator
#[derive(Clone, Copy)]
pub enum SOSC {
    RefClock(u32),
    Oscillator(u32),
}
impl SOSC {
    #[inline(always)]
    pub const fn freq(&self) -> u32 {
        match self {
            SOSC::RefClock(f) => *f,
            SOSC::Oscillator(f) => *f,
        }
    }

    #[inline(always)]
    const fn range(&self) -> Result<u8, SCGError> {
        match self.freq() {
            8_000_000..16_000_000 => Ok(0),
            16_000_000..25_000_000 => Ok(1),
            25_000_000..40_000_000 => Ok(2),
            40_000_000..50_000_000 => Ok(3),
            _ => Err(SCGError::InvalidValue),
        }
    }

    fn enable_sosc(scg: crate::pac::scg::SCG, sosc: SOSC, stop_en: bool) -> Result<(), SCGError> {
        #[cfg(feature = "mcxa2")]
        scg.LDOCSR().modify(|r| r.set_LDOEN(true));

        let range = sosc.range()?;
        scg.SOSCCFG().write(|r| {
            r.set_EREFS(matches!(sosc, SOSC::Oscillator(_)));
            r.set_RANGE(range);
        });
        scg.SOSCCSR().modify(|r| r.set_LK(false));
        scg.SOSCCSR().modify(|r| {
            r.set_SOSCEN(true);
            r.set_SOSCSTEN(stop_en);
        });
        scg.SOSCCSR().modify(|r| r.set_LK(true));

        while !scg.SOSCCSR().read().SOSCVLD() {}
        if scg.SOSCCSR().read().SOSCERR() {
            return Err(SCGError::SOSCErr);
        }

        Ok(())
    }
    fn disable_sosc(scg: crate::pac::scg::SCG) -> Result<(), SCGError> {
        if scg.SOSCCSR().read().SOSCSEL() {
            return Err(SCGError::Busy);
        }

        scg.SOSCCSR().modify(|r| r.set_LK(false));
        scg.SOSCCSR().modify(|r| r.set_SOSCEN(false));
        scg.SOSCCSR().modify(|r| r.set_LK(true));

        Ok(())
    }
}

#[derive(Clone, Copy, Default)]
pub enum MainClockSource {
    SOSC = 1,
    SIRC = 2,
    #[default]
    FIRC = 3,
    ROSC = 4,
    APLL = 5,
    SPLL = 6,
    UPLL = 7,
    TRO = 8,
}
impl MainClockSource {
    pub const fn valid(&self) -> bool {
        match self {
            MainClockSource::SOSC
            | MainClockSource::SIRC
            | MainClockSource::FIRC
            | MainClockSource::ROSC => true,

            #[cfg(feature = "mcxa2")]
            MainClockSource::SPLL => true,

            _ => false,
        }
    }
}

#[derive(Clone, Copy)]
pub struct SCGConfig {
    pub sosc: Option<SOSC>,
    pub sosc_stop_en: bool,

    pub sirc_stop_en: bool,
    pub sirc_clk_en: bool,

    pub firc: Option<FIRC>,
    pub firc_stop_en: bool,
    pub firc_sclk_en: bool,
    pub firc_fclk_en: bool,

    #[cfg(feature = "mcxa2")]
    pub spll: Option<PllConfig>,

    pub main_clock_source: MainClockSource,
}
impl Default for SCGConfig {
    fn default() -> Self {
        Self {
            sosc: None,
            sosc_stop_en: false,
            sirc_stop_en: false,
            sirc_clk_en: true,
            firc: Some(FIRC::default()),
            firc_stop_en: false,
            firc_sclk_en: true,
            firc_fclk_en: true,
            spll: None,
            main_clock_source: MainClockSource::default(),
        }
    }
}
impl SCGConfig {
    pub fn valid(&self) -> bool {
        if !self.main_clock_source.valid() {
            return false;
        }

        true
    }
}
