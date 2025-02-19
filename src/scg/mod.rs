//! System Clock Generator

use crate::{
    pac::scg::Instance,
    port::scg::{Pin, EXTAL as SigEXTAL, XTAL as SigXTAL},
};

mod firc;
use firc::FIRC;

#[derive(Clone, Copy, Debug)]
pub enum SCGError {
    MissingPins,
    Busy,

    InvalidValue,
    InvalidConfig,

    SOSCErr,
    SIRCErr,
    FIRCErr,
}

pub struct SCG<const N: u8, PINS> {
    scg: Instance<N>,
    pins: PINS,
    has_pins: bool,
    config: Config,
}
unsafe impl<const N: u8, PINS> Send for SCG<N, PINS> {}
unsafe impl<const N: u8, PINS> Sync for SCG<N, PINS> {}
impl<const N: u8, EXTAL, XTAL> SCG<N, Pins<EXTAL, XTAL>>
where
    EXTAL: Pin<Signal = SigEXTAL>,
    XTAL: Pin<Signal = SigXTAL, Module = EXTAL::Module>,
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
            config: Config::default(),
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
            config: Config::default(),
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

        if scg.CSR().read().SCS() == self.config.main_clock_source as u8 {
            return Ok(());
        }
        scg.RCCR()
            .write(|r| r.set_SCS(self.config.main_clock_source as u8));
        while scg.CSR().read().SCS() != self.config.main_clock_source as u8 {}

        Ok(())
    }

    pub fn back_to_default(&mut self) -> Result<(), SCGError> {
        self.config = Config::default();
        self.freeze()
    }
}

pub struct Pins<EXTAL, XTAL>
where
    EXTAL: Pin<Signal = SigEXTAL>,
    XTAL: Pin<Signal = SigXTAL, Module = EXTAL::Module>,
{
    extal: EXTAL,
    xtal: Option<XTAL>,
}
unsafe impl<EXTAL: Pin<Signal = SigEXTAL>, XTAL: Pin<Signal = SigXTAL, Module = EXTAL::Module>> Send
    for Pins<EXTAL, XTAL>
{
}
unsafe impl<EXTAL: Pin<Signal = SigEXTAL>, XTAL: Pin<Signal = SigXTAL, Module = EXTAL::Module>> Sync
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
pub struct Config {
    pub sosc: Option<SOSC>,
    pub sosc_stop_en: bool,

    pub sirc_stop_en: bool,
    pub sirc_clk_en: bool,

    pub firc: Option<FIRC>,
    pub firc_stop_en: bool,
    pub firc_sclk_en: bool,
    pub firc_fclk_en: bool,

    pub main_clock_source: MainClockSource,
}
impl Default for Config {
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
            main_clock_source: MainClockSource::default(),
        }
    }
}
impl Config {
    pub fn valid(&self) -> bool {
        if !self.main_clock_source.valid() {
            return false;
        }

        true
    }
}
