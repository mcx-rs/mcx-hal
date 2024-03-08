//! # SYSCON based clock control
//! MCX N Series uses SYSCON to control peripheral clocks.

pub trait ClockExt: crate::sealed::Sealed {
    /// enable Peripheral clock
    fn enable(&self);

    /// disable Peripheral clock
    fn disable(&self);
}

/// implement ClockExt
///
/// TODO: add a virtual tag for virtual peripheral which does not have a clock control
macro_rules! impl_clockext {
    ($([$name:tt, [$(($ahb:expr, $bit:expr),)*]]), +,) => {
        use $crate::sealed::Sealed;

        $(
            impl Sealed for $crate::pac::$name {}

            #[allow(unused_variables)]
            impl ClockExt for $crate::pac::$name {
                #[inline]
                fn enable(&self) {
                    let syscon = $crate::pac::SYSCON0::ptr();
                    $(
                        unsafe { (*syscon).ahbclkctrlset($ahb).write(|w| w.bits(1 << $bit)); }
                    )*
                }

                #[inline]
                fn disable(&self) {
                    let syscon = $crate::pac::SYSCON0::ptr();
                    $(
                        unsafe { (*syscon).ahbclkctrlclr($ahb).write(|w| w.bits(1 << $bit)); }
                    )*
                }
            }
        )+

        #[allow(non_snake_case)]
        /// Peripheral clock control
        pub mod PeripheralClocks {
            $(
                #[allow(non_snake_case, unused_variables)]
                /// Peripheral clock control
                pub mod $name {
                    /// enable Peripheral clock
                    pub fn enable() {
                        let syscon = $crate::pac::SYSCON0::ptr();
                        $(
                            unsafe { (*syscon).ahbclkctrlset($ahb).write(|w| w.bits(1 << $bit)); }
                        )*
                    }

                    /// disable Peripheral clock
                    pub fn disable() {
                        let syscon = $crate::pac::SYSCON0::ptr();
                        $(
                            unsafe { (*syscon).ahbclkctrlclr($ahb).write(|w| w.bits(1 << $bit)); }
                        )*
                    }
                }
            )+
        }
    };
}

pub use crate::chip::clock::PeripheralClocks;
pub(crate) use impl_clockext;

pub enum SystickClockSource {
    Disable,
    MainClock(u8),
    Clk1M,
    LPOSC,
}

pub fn enable_systick(num: usize, source: SystickClockSource) {
    let syscon = unsafe { crate::pac::SYSCON0::steal() };
    match source {
        SystickClockSource::Disable => syscon.systickclksel(num).write(|w| w.sel().bits(0b111)),
        SystickClockSource::MainClock(div) => {
            syscon
                .systickclkdiv(num)
                .write(|w| unsafe { w.div().bits(div).halt().run() });
            while syscon.systickclkdiv(num).read().unstab().is_ongoing() {}
            syscon.systickclksel(num).write(|w| w.sel().bits(0b000));
        }
        SystickClockSource::Clk1M => {
            syscon.systickclksel(num).write(|w| w.sel().bits(0b001));
        }
        SystickClockSource::LPOSC => unimplemented!(),
    }
}
