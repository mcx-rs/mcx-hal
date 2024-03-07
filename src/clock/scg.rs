//! # SCG
//! System Clock Generator

// TODO: move chip relatived things to chip module

use crate::{
    pac::{self, SCG0},
    power::{self, RunMode},
};

/// System Clock Source
#[derive(Clone, Copy, Debug)]
pub enum MainClockSource {
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

/// FIRC Frequency select
#[derive(Clone, Copy, Debug)]
pub enum FIRCRange {
    FIRC144M,
    FIRC48M,
}

/// Clocks Configuration
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// SOSC Mode
    pub sosc_mode: Option<SOSCMode>,
    /// FIRC range
    pub firc_range: Option<FIRCRange>,

    /// Main Clock source selection
    pub main_clock_source: MainClockSource,

    /// enable SIRC clock to peripherals
    pub sirc_clk_periph_en: bool,
    /// enable FIRC FCLK to peripherals
    pub firc_fclk_periph_en: bool,
    /// enable FIRC SCLK to peripherals
    pub firc_sclk_periph_en: bool,

    pub ahbclkdiv: u8,
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

    pub const fn new_fro12m() -> Self {
        Self {
            sosc_mode: None,
            firc_range: None,

            main_clock_source: MainClockSource::SIRC,

            sirc_clk_periph_en: true,
            firc_fclk_periph_en: false,
            firc_sclk_periph_en: false,

            ahbclkdiv: 0,
        }
    }

    pub const fn new_frohf48m() -> Self {
        Self {
            sosc_mode: None,
            firc_range: Some(FIRCRange::FIRC48M),

            main_clock_source: MainClockSource::FIRC,

            sirc_clk_periph_en: true,
            firc_fclk_periph_en: false,
            firc_sclk_periph_en: false,

            ahbclkdiv: 0,
        }
    }

    pub fn set_main_clock_source(&mut self, source: MainClockSource) {
        self.main_clock_source = source;
    }

    pub fn use_sosc(&mut self) {
        self.set_main_clock_source(MainClockSource::SOSC);
    }

    pub fn use_sirc(&mut self) {
        self.set_main_clock_source(MainClockSource::SIRC);
    }

    pub fn use_firc(&mut self) {
        self.set_main_clock_source(MainClockSource::FIRC);
    }

    pub fn set_firc_range(&mut self, range: Option<FIRCRange>) {
        self.firc_range = range;
    }

    pub fn is_valid(&self) -> bool {
        // Check System Clock
        match self.main_clock_source {
            MainClockSource::SIRC => {}
            MainClockSource::FIRC => {
                if self.firc_range.is_none() {
                    return false;
                }
            }
            MainClockSource::SOSC => {
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

    pub fn get_main_clk(&self) -> Option<u32> {
        match self.main_clock_source {
            MainClockSource::SOSC => self.get_sosc_clk(),
            MainClockSource::FIRC => self.get_firc_clk(),
            MainClockSource::SIRC => self.get_sirc_12mhz(),
            MainClockSource::Stop => None,
            _ => todo!(),
        }
    }

    pub fn get_system_clk(&self) -> Option<u32> {
        match self.get_main_clk() {
            Some(clk) => Some(clk / self.ahbclkdiv as u32),
            None => None,
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
    run_mode: power::RunMode,

    pub config: Config,
}

impl Clocks {
    pub fn constrain(scg: SCG0) -> Self {
        Self {
            rb: scg,
            config: Config::default(),
            run_mode: power::RunMode::MidDrive,
        }
    }

    pub fn use_config(&mut self, config: Config) {
        self.config = config;
    }

    pub fn freeze(&mut self) {
        assert!(self.config.is_valid());
        self.setup_sosc();
        self.setup_irc();

        match self.config.main_clock_source {
            MainClockSource::SOSC => {
                assert!(self.config.sosc_mode.is_some());
                self.rb.rccr().modify(|_r, w| w.scs().sosc());
                while !self.rb.csr().read().scs().is_sosc() {}
            }
            MainClockSource::SIRC => {
                self.rb.rccr().modify(|_r, w| w.scs().sirc());
                while !self.rb.csr().read().scs().is_sirc() {}
            }
            MainClockSource::FIRC => {
                assert!(self.config.firc_range.is_some());
                self.rb.rccr().modify(|_r, w| w.scs().firc());
                while !self.rb.csr().read().scs().is_firc() {}
            }
            _ => {
                todo!()
            }
        }

        let run_mode = match self.config.get_system_clk().unwrap() {
            0..=50_000_000u32 => RunMode::MidDrive,
            50_000_001u32..=100_000_000u32 => RunMode::StandardDrive,
            100_000_001u32..=150_000_000u32 => RunMode::OverDrive,
            _ => panic!(),
        };
        self.setup_run_mode(run_mode);
        self.run_mode = run_mode;
    }

    pub fn get_current_run_mode(&self) -> power::RunMode {
        self.run_mode
    }

    fn setup_run_mode(&mut self, run_mode: power::RunMode) {
        assert!(run_mode != power::RunMode::UnderDrive);
        if run_mode == self.run_mode {
            return;
        }
        if run_mode > self.run_mode {
            Self::increase_run_mode(run_mode, self.config.get_main_clk().unwrap());
        } else {
            Self::decrease_run_mode(run_mode, self.config.get_main_clk().unwrap());
        }
    }

    fn increase_run_mode(run_mode: power::RunMode, clk: u32) {
        let spc = unsafe { pac::SPC0::steal() };
        spc.active_cfg().modify(|_r, w| unsafe {
            w.coreldo_vdd_lvl()
                .bits(run_mode as u8)
                .dcdc_vdd_lvl()
                .bits(run_mode as u8)
        });
        while spc.sc().read().busy().is_busy_yes() {}
        Self::setup_flash_access_cycles(run_mode, clk);
        spc.sramctl()
            .modify(|_r, w| unsafe { w.vsm().bits(run_mode as u8) });
        spc.sramctl().modify(|_r, w| w.req().req_yes());
        while spc.sramctl().read().ack().is_ack_no() {}
        spc.sramctl().modify(|_r, w| w.req().req_no());
    }

    fn decrease_run_mode(run_mode: power::RunMode, clk: u32) {
        let spc = unsafe { pac::SPC0::steal() };
        Self::setup_flash_access_cycles(run_mode, clk);
        spc.sramctl()
            .modify(|_r, w| unsafe { w.vsm().bits(run_mode as u8) });
        spc.sramctl().modify(|_r, w| w.req().req_yes());
        while spc.sramctl().read().ack().is_ack_no() {}
        spc.sramctl().modify(|_r, w| w.req().req_no());
        spc.active_cfg().modify(|_r, w| unsafe {
            w.coreldo_vdd_lvl()
                .bits(run_mode as u8)
                .dcdc_vdd_lvl()
                .bits(run_mode as u8)
        });
        while spc.sc().read().busy().is_busy_yes() {}
    }

    fn setup_sosc(&mut self) {
        let sosc_mode = match self.config.sosc_mode {
            None => {
                return;
            }
            Some(val) => val,
        };
        let range = self.config.sosc_range();
        assert!(range != u8::max_value());

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

    fn setup_irc(&mut self) {
        self.rb.sirccsr().modify(|_r, w| w.lk().write_enabled());
        if self.config.sirc_clk_periph_en {
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
            match self.config.firc_range {
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
            if self.config.firc_sclk_periph_en {
                w.firc_sclk_periph_en().enabled();
            }
            if self.config.firc_fclk_periph_en {
                w.firc_fclk_periph_en().enabled();
            }
            if self.config.firc_range.is_some() {
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

    fn setup_flash_access_cycles(run_mode: power::RunMode, clk: u32) {
        unsafe {
            pac::FMU0::steal().fctrl().modify(|_r, w| match run_mode {
                power::RunMode::MidDrive => match clk {
                    0..=24_000_000u32 => w.rwsc().bits(0),
                    24_000_001u32..=50_000_000u32 => w.rwsc().bits(1),
                    _ => panic!(),
                },
                power::RunMode::StandardDrive => match clk {
                    0..=36_000_000u32 => w.rwsc().bits(0),
                    36_000_001u32..=64_000_000u32 => w.rwsc().bits(1),
                    64_000_001u32..=100_000_000u32 => w.rwsc().bits(2),
                    _ => panic!(),
                },
                power::RunMode::OverDrive => match clk {
                    0..=36_000_000u32 => w.rwsc().bits(0),
                    36_000_001u32..=64_000_000u32 => w.rwsc().bits(1),
                    64_000_001u32..=100_000_000u32 => w.rwsc().bits(2),
                    100_000_001u32..=150_000_000u32 => w.rwsc().bits(3),
                    _ => panic!(),
                },
                _ => panic!(),
            })
        };
    }
}
