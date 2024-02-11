//! # SYSCON based clock control
//! MCX N Series uses SYSCON to control peripheral clocks.

pub trait ClockExt {
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
        $(
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

pub(crate) use impl_clockext;

pub use crate::chip::clock::PeripheralClocks;
