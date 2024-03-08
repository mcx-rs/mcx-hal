//! # SYSCON Dividers and Clock Source control
//!

pub mod systick {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum ClockSource {
        MainClock(u8),
        Clk1M,
        LpOsc,
        NoClock,
    }

    impl Into<u8> for ClockSource {
        fn into(self) -> u8 {
            match self {
                ClockSource::MainClock(_) => 0b000,
                ClockSource::Clk1M => 0b001,
                ClockSource::LpOsc => 0b010,
                ClockSource::NoClock => 0b111,
            }
        }
    }

    pub fn setup(n: usize, source: ClockSource) {
        let syscon = unsafe { crate::pac::SYSCON0::steal() };
        match source {
            ClockSource::MainClock(div) => {
                syscon
                    .systickclkdiv(n)
                    .write(|w| unsafe { w.div().bits(div).halt().run() });
                while syscon.systickclkdiv(n).read().unstab().is_ongoing() {}
            }
            _ => {}
        }
        syscon
            .systickclksel(n)
            .write(|w| w.sel().bits(source.into()));
    }

    pub fn get_source(n: usize) -> ClockSource {
        let syscon = unsafe { crate::pac::SYSCON0::steal() };
        let source = syscon.systickclksel(n).read().sel().bits();
        match source {
            0 => {
                let div = syscon.systickclkdiv(n).read().div().bits();
                ClockSource::MainClock(div)
            }
            1 => ClockSource::Clk1M,
            2 => ClockSource::LpOsc,
            _ => ClockSource::NoClock,
        }
    }
}

pub mod clkout {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum ClockSource {
        MainClock,
        APll,
        ClkIn,
        FroHF,
        Fro12M,
        SPll,
        LpOsc,
        UPll,
        NoClock,
    }

    impl Into<u8> for ClockSource {
        fn into(self) -> u8 {
            match self {
                ClockSource::MainClock => 0b0000,
                ClockSource::APll => 0b0001,
                ClockSource::ClkIn => 0b0010,
                ClockSource::FroHF => 0b0011,
                ClockSource::Fro12M => 0b0100,
                ClockSource::SPll => 0b0101,
                ClockSource::LpOsc => 0b0110,
                ClockSource::UPll => 0b0111,
                ClockSource::NoClock => 0b1111,
            }
        }
    }

    pub fn setup(source: ClockSource, div: u8) {
        let syscon = unsafe { crate::pac::SYSCON0::steal() };
        syscon.clkoutsel().write(|w| w.sel().bits(source.into()));
        syscon.clkoutdiv().write(|w| unsafe { w.div().bits(div) });
    }

    pub fn get_source() -> ClockSource {
        let syscon = unsafe { crate::pac::SYSCON0::steal() };
        let source = syscon.clkoutsel().read().sel().bits();
        match source {
            0 => ClockSource::MainClock,
            1 => ClockSource::APll,
            2 => ClockSource::ClkIn,
            3 => ClockSource::FroHF,
            4 => ClockSource::Fro12M,
            5 => ClockSource::SPll,
            6 => ClockSource::LpOsc,
            7 => ClockSource::UPll,
            _ => ClockSource::NoClock,
        }
    }

    pub fn get_div() -> u8 {
        let syscon = unsafe { crate::pac::SYSCON0::steal() };
        syscon.clkoutdiv().read().div().bits()
    }

    pub fn halt() {
        let syscon = unsafe { crate::pac::SYSCON0::steal() };
        syscon.clkoutdiv().modify(|_r, w| w.halt().halt());
    }

    pub fn run() {
        let syscon = unsafe { crate::pac::SYSCON0::steal() };
        syscon.clkoutdiv().modify(|_r, w| w.halt().run());
        while syscon.clkoutdiv().read().unstab().is_ongoing() {}
    }
}
