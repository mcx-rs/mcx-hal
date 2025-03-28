//! System Clock Generator (SCG)
//!
//! # Clock Diagram
//!
//!                       firc_sclk_en                                               
//!                           ┌──┐                                                   
//!                     ┌─────┼──┼──────┬──────────────────────────────► FIRC_SCLK   
//!                     │     └──┘      │                                            
//!             firc_en │ firc_fclk_en  │                                            
//! ┌──────┐      ┌──┐  │     ┌──┐      │                                            
//! │ FIRC ├──────┼──┼──┴─────┼──┼──────┼──────────────────────────────► FIRC_FCLK   
//! └──────┘      └──┘        └──┘      │                                            
//!                   ┌─────────────────┼──────────────────────────────► SIRC_1M_CLK
//!                   │ sirc_12m_clk_en │                                            
//! ┌──────┐ 12M      │       ┌──┐      │                                            
//! │ SIRC ├──────────┴───────┼──┼──────┼┬─────────────────────────────► SIRC_12M_CLK
//! └──────┘                  └──┘      ││                                           
//!                       sosc_en       ││                                           
//!  OSC      ┌──────┐     ┌──┐         ││                                           
//!   or  ───►├─SOSC─┼─────┼──┼────────┬┼┼─────────────────────────────► SOSC_CLK / CLK_IN      
//! RefClk    └──────┘     └──┘        │││                                           
//!                                    │││    ┌──────┐                               
//!                                    ├┼┼───►│      │                               
//!                                    │├┼───►│ SPLL ├─────────────────► SPLL_CLK            
//!                                    ││├───►│      │                               
//!                                    │││    └──────┘                               
//!                                    │││    ┌──────┐                               
//!                                    └┼┼───►│      │                               
//!                                     └┼───►│ APLL ├─────────────────► APLL_CLK            
//!                                      └───►│      │                               
//!                                           └──────┘                               

mod firc;
mod sosc;
pub use firc::FIRC;
pub use sosc::SOSC;

use crate::port::{
    consts::Const,
    scg::{prepare, Pin, EXTAL48M, XTAL48M},
};
type Instance = crate::pac::scg::Instance<0>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SCGError {
    OutOfRange,

    SOSCBusy,
    SOSCError,

    SIRCBusy,
    SIRCError,

    FIRCBusy,
    FIRCError,

    InvalidConfig,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub sosc: Option<SOSC>,
    pub sosc_stop_en: bool,

    pub sirc_12m_clk_en: bool,
    pub sirc_stop_en: bool,

    pub firc: Option<FIRC>,
    pub firc_sclk_en: bool,
    pub firc_fclk_en: bool,
    pub firc_stop_en: bool,

    pub main_clock_source: MainClockSource,
}
impl Default for Config {
    fn default() -> Self {
        todo!();
    }
}
impl Config {
    pub const fn valid(&self) -> bool {
        if !self.main_clock_source.valid() {
            return false;
        }

        if self.sosc.is_none() && matches!(self.main_clock_source, MainClockSource::SOSC) {
            return false;
        }
        if !self.sirc_12m_clk_en && matches!(self.main_clock_source, MainClockSource::SIRC) {
            return false;
        }
        if !self.firc_fclk_en && matches!(self.main_clock_source, MainClockSource::FIRC) {
            return false;
        }

        true
    }
}

/// SCG Instance.
pub struct SCG<PINS> {
    scg: Instance,
    pins: PINS,
}
impl<EXTAL, XTAL> SCG<Pins<EXTAL, XTAL>>
where
    EXTAL: Pin<Signal = EXTAL48M, Module = Const<0>>,
    XTAL: Pin<Signal = XTAL48M, Module = EXTAL::Module>,
{
    pub fn new(scg: Instance, mut pins: Pins<EXTAL, XTAL>) -> Self {
        prepare(&mut pins.extal);
        if let Some(pin) = &mut pins.xtal {
            prepare(pin);
        }
        Self { scg, pins }
    }
}
impl SCG<()> {
    pub fn without_pins(scg: Instance) -> Self {
        Self { scg, pins: () }
    }
}
impl<PINS> SCG<PINS> {
    /// Release SCG.
    ///
    /// # Safety
    /// Why need this? There's only one SCG instance per chip, and it is related to clocks.
    pub unsafe fn release(self) -> (Instance, PINS) {
        (self.scg, self.pins)
    }

    /// Freeze SCG with given clock config.
    pub fn freeze(&mut self, config: &Config) -> Result<(), SCGError> {
        if !config.valid() {
            return Err(SCGError::InvalidConfig);
        }

        // configure SOSC
        match config.sosc {
            Some(sosc) => SOSC::enable(self.scg.regs(), sosc, config.sosc_stop_en)?,
            None => SOSC::disable(self.scg.regs())?,
        }

        // configure SIRC
        if !config.sirc_12m_clk_en && self.scg.regs().SIRCCSR().read().SIRCSEL() {
            return Err(SCGError::SIRCBusy);
        }
        self.scg.regs().SIRCCSR().modify(|r| r.set_LK(false));
        self.scg.regs().SIRCCSR().modify(|r| {
            r.set_SIRCSTEN(config.sirc_stop_en);
            r.set_SIRC_CLK_PERIPH_EN(config.sirc_12m_clk_en);
            r.set_LK(true);
        });
        while !self.scg.regs().SIRCCSR().read().SIRCVLD() {}
        if self.scg.regs().SIRCCSR().read().SIRCERR() {
            return Err(SCGError::SIRCError);
        }

        // configure FIRC
        match config.firc {
            Some(firc) => FIRC::enable(
                self.scg.regs(),
                firc,
                config.firc_stop_en,
                config.firc_fclk_en,
                config.firc_sclk_en,
            )?,
            None => FIRC::disable(self.scg.regs())?,
        }

        // Switch Main Clock Source
        self.scg
            .regs()
            .RCCR()
            .write(|r| r.set_SCS(config.main_clock_source as u8));
        while self.scg.regs().CSR().read().SCS() != config.main_clock_source as u8 {}

        Ok(())
    }
}

pub struct Pins<EXTAL, XTAL>
where
    EXTAL: Pin<Signal = EXTAL48M, Module = Const<0>>,
    XTAL: Pin<Signal = XTAL48M, Module = EXTAL::Module>,
{
    pub extal: EXTAL,
    pub xtal: Option<XTAL>,
}
