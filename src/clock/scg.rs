//! # SCG

use crate::pac::SCG0;

/// System Clock Source
#[derive(Clone, Copy, Debug)]
pub enum SystemClockSource {
    SOSC,
    SIRC,
    FIRC,
    ROSC,
    APLL,
    SPLL,
    UPLL,
    Stop,
}

/// SOSC Mode
#[derive(Clone, Copy, Debug)]
pub enum SOSCMode {
    /// external reference clock
    RefClock(u32),
    /// internal crystal oscillator
    Oscillator(u32),
}

#[derive(Clone, Copy, Debug)]
pub enum FIRCRange {
    FIRC144M,
    FIRC48M,
}

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub sosc_mode: Option<SOSCMode>,
    pub firc_range: Option<FIRCRange>,

    pub system_clock_source: SystemClockSource,

    pub sirc_clk_periph_en: bool,
    pub firc_fclk_periph_en: bool,
    pub firc_sclk_periph_en: bool,
}

impl Default for Config {
    fn default() -> Self {
        todo!()
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_system_clock_source(mut self, source: SystemClockSource) -> Self {
        self.system_clock_source = source;
        self
    }

    pub fn use_sosc(self) -> Self {
        self.set_system_clock_source(SystemClockSource::SOSC)
    }

    pub fn use_sirc(self) -> Self {
        self.set_system_clock_source(SystemClockSource::SIRC)
    }

    pub fn use_firc(self) -> Self {
        self.set_system_clock_source(SystemClockSource::FIRC)
    }

    pub fn set_firc_range(mut self, range: Option<FIRCRange>) -> Self {
        self.firc_range = range;
        self
    }

    pub fn is_valid(&self) -> bool {
        // Check System Clock
        match self.system_clock_source {
            SystemClockSource::SIRC => {}
            SystemClockSource::FIRC => {
                if self.firc_range.is_none() {
                    return false;
                }
            }
            SystemClockSource::SOSC => {
                if self.sosc_mode.is_none() {
                    return false;
                }
            }
            _ => todo!(),
        }

        // Check SOSC
        if self.sosc_mode.is_some() {
            let freq = match self.sosc_mode {
                Some(SOSCMode::RefClock(freq)) => freq,
                Some(SOSCMode::Oscillator(freq)) => freq,
                None => unreachable!(),
            };
            if freq < 16_000_000u32 || freq > 66_000_000u32 {
                return false;
            }
        }

        true
    }

    pub fn get_sirc_1mhz(&self) -> Option<u32> {
        Some(1_000_000u32)
    }

    pub fn get_sirc_12mhz(&self) -> Option<u32> {
        if self.sirc_clk_periph_en {
            Some(12_000_000u32)
        } else {
            None
        }
    }

    pub fn get_firc_144mhz(&self) -> Option<u32> {
        if self.firc_fclk_periph_en {
            Some(144_000_000u32)
        } else {
            None
        }
    }

    pub fn get_firc_48mhz(&self) -> Option<u32> {
        if self.firc_sclk_periph_en {
            Some(48_000_000u32)
        } else {
            None
        }
    }

    pub fn get_firc_clk(&self) -> Option<u32> {
        match self.firc_range {
            None => None,
            Some(FIRCRange::FIRC144M) => Some(144_000_000u32),
            Some(FIRCRange::FIRC48M) => Some(48_000_000u32),
        }
    }

    pub fn get_sosc_clk(&self) -> Option<u32> {
        match self.sosc_mode {
            None => None,
            Some(SOSCMode::Oscillator(freq)) => Some(freq),
            Some(SOSCMode::RefClock(freq)) => Some(freq),
        }
    }

    fn sosc_range(&self) -> u8 {
        let freq = match self.sosc_mode {
            None => {
                return u8::max_value();
            }
            Some(SOSCMode::Oscillator(freq)) => freq,
            Some(SOSCMode::RefClock(freq)) => freq,
        };

        let range: u8 = if 16_000_000u32 <= freq && freq < 20_000_000u32 {
            0
        } else if 20_000_000u32 <= freq && freq < 30_000_000u32 {
            1
        } else if 30_000_000u32 <= freq && freq < 50_000_000u32 {
            2
        } else if 50_000_000u32 <= freq && freq <= 66_000_000u32 {
            3
        } else {
            u8::max_value()
        };
        range
    }
}

pub struct Clocks {
    rb: SCG0,
}

impl Clocks {
    pub fn constrain(scg: SCG0) -> Self {
        Self { rb: scg }
    }

    pub fn freeze(&mut self, config: &Config) {
        assert!(config.is_valid());
        self.setup_sosc(config);
        self.setup_irc(config);

        match config.system_clock_source {
            SystemClockSource::SOSC => {
                assert!(config.sosc_mode.is_some());
                self.rb.rccr().modify(|_r, w| w.scs().sosc());
                while !self.rb.csr().read().scs().is_sosc() {}
            }
            SystemClockSource::SIRC => {
                self.rb.rccr().modify(|_r, w| w.scs().sirc());
                while !self.rb.csr().read().scs().is_sirc() {}
            }
            SystemClockSource::FIRC => {
                assert!(config.firc_range.is_some());
                self.rb.rccr().modify(|_r, w| w.scs().firc());
                while !self.rb.csr().read().scs().is_firc() {}
            }
            _ => {
                todo!()
            }
        }
    }

    fn setup_sosc(&mut self, config: &Config) {
        let sosc_mode = match config.sosc_mode {
            None => {
                return;
            }
            Some(val) => val,
        };
        let freq = match sosc_mode {
            SOSCMode::Oscillator(freq) => freq,
            SOSCMode::RefClock(freq) => freq,
        };
        assert!(16_000_000u32 <= freq && freq <= 66_000_000u32);
        let range: u8 = if 16_000_000u32 <= freq && freq < 20_000_000u32 {
            0
        } else if 20_000_000u32 <= freq && freq < 30_000_000u32 {
            1
        } else if 30_000_000u32 <= freq && freq < 50_000_000u32 {
            2
        } else {
            3
        };

        self.rb.ldocsr().modify(|_r, w| w.ldoen().enabled());
        self.rb.sosccfg().modify(|_r, w| {
            match sosc_mode {
                SOSCMode::Oscillator(_) => w.erefs().internal(),
                SOSCMode::RefClock(_) => w.erefs().external(),
            }
            .range()
            .bits(range)
        });
        self.rb.sosccsr().modify(|_r, w| w.lk().write_enabled());
        self.rb.sosccsr().modify(|_r, w| w.soscen().enabled());
        while self.rb.sosccsr().read().soscvld().is_disabled() {}
        assert!(self.rb.sosccsr().read().soscerr().is_enabled_and_error());
        self.rb.sosccsr().modify(|_r, w| w.lk().write_disabled());
    }

    fn setup_irc(&mut self, config: &Config) {
        self.rb.sirccsr().modify(|_r, w| w.lk().write_enabled());
        if config.sirc_clk_periph_en {
            self.rb
                .sirccsr()
                .modify(|_r, w| w.sirc_clk_periph_en().enabled());
        }
        while self
            .rb
            .sirccsr()
            .read()
            .sircvld()
            .is_disabled_or_not_valid()
        {}
        assert!(self.rb.sirccsr().read().sircerr().is_error_not_detected());
        self.rb.sirccsr().modify(|_r, w| w.lk().write_disabled());

        self.rb.firccfg().modify(|_r, w| {
            match config.firc_range {
                Some(FIRCRange::FIRC144M) => {
                    w.range().firc_144mhz();
                }
                Some(FIRCRange::FIRC48M) => {
                    w.range().firc_48mhz();
                }
                None => {}
            };
            w
        });
        self.rb.firccsr().modify(|_r, w| w.lk().write_enabled());
        self.rb.firccsr().modify(|_r, w| {
            if config.firc_sclk_periph_en {
                w.firc_sclk_periph_en().enabled();
            }
            if config.firc_fclk_periph_en {
                w.firc_fclk_periph_en().enabled();
            }
            if config.firc_range.is_some() {
                w.fircen().enabled();
            }
            w
        });
        while self
            .rb
            .firccsr()
            .read()
            .fircvld()
            .is_not_enabled_or_not_valid()
        {}
        assert!(self.rb.firccsr().read().fircerr().is_error_not_detected());
        self.rb.firccsr().modify(|_r, w| w.lk().write_disabled());
    }
}
