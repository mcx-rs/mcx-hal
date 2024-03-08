//! # General Purpose Input/Output
//! GPIO driver for NXP MCX series MCUs.
//!
//! ## Usage
//!
//! ``` no_run
//! use mcx_hal::{self as hal, pac};
//!
//! let dp = pac::Peripherals::take().unwrap();
//! let gpio0 = hal::gpio::gpio0::split(dp.GPIO0, dp.PORT0);
//! let mut led_r = gpio0.pio0_10.into_push_pull_output();
//!
//! led_r.set_high();
//! led_r.set_low();
//! led_r.toggle();
//! ```
//!

use core::marker::PhantomData;

/// Type state Muxed
pub struct Muxed<MODE, const MUX: u8>(PhantomData<MODE>);
/// Type state Input
pub struct Input<MODE>(PhantomData<MODE>);
/// Type state Output
pub struct Output<MODE>(PhantomData<MODE>);
/// Type state Analog
pub struct Analog;
pub struct Floating;
pub struct PullUp;
pub struct PullDown;
pub struct PushPull;
pub struct OpenDrain;

#[derive(Clone, Copy, Debug)]
pub enum GPIOInterruptSource {
    Logic0 = 0b1000,
    RisingEdge = 0b01001,
    FallingEdge = 0b1010,
    EitherEdge = 0b1011,
    Logic1 = 0b1100,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOInterruptSelect {
    IRQ0 = 0,
    IRQ1 = 1,
}

impl Into<bool> for GPIOInterruptSelect {
    fn into(self) -> bool {
        self == GPIOInterruptSelect::IRQ1
    }
}

pub struct Pin<MODE, const PORT: u8, const PIN: u8> {
    pub(crate) _mode: PhantomData<MODE>,
}

impl<MODE, const PORT: u8, const PIN: u8> Pin<MODE, PORT, PIN> {
    /// Pin's port number
    pub const fn port() -> u8 {
        PORT
    }

    /// Pin's pin number
    pub const fn pin() -> u8 {
        PIN
    }
}

macro_rules! gpio {
    (
        index: $index:expr,
            $( [pin: $pin:expr, [ $($mux:expr),+ ], $default_mode:ty $(, pfe: $pfe:expr)? $(, pv: $pv:expr)?])+
    ) => {
        paste::paste! {
            pub mod [< gpio $index >] {
                use core::marker::PhantomData;
                use core::convert::Infallible;
                use $crate::pac::{[< GPIO $index >] as GPIO, [< PORT $index >] as PORT};
                use $crate::clock::PeripheralClocks;
                use $crate::gpio::{gpio, Pin, Muxed, Input, Output, Analog, Floating, PullUp, PullDown, PushPull, OpenDrain};
                use $crate::gpio::{GPIOInterruptSource, GPIOInterruptSelect};
                use embedded_hal::digital::{ErrorType, InputPin, OutputPin, StatefulOutputPin};

                pub fn split(gpio: GPIO, port: PORT) -> Parts { Parts::new(gpio, port) }

                pub struct Parts {
                    $( pub [< pio $index _ $pin >]: [< PIO $index _ $pin >]<$default_mode>, )+
                }

                impl Parts {
                    pub fn new(_gpio: GPIO, _port: PORT) -> Self {
                        PeripheralClocks::[< GPIO $index >]::enable();
                        PeripheralClocks::[< PORT $index >]::enable();
                        Self {
                            $( [< pio $index _ $pin >]: [< PIO $index _ $pin >] { _mode: PhantomData }, )+
                        }
                    }
                }

                $(
                    pub struct [< PIO $index _ $pin >]<MODE> {
                        _mode: PhantomData<MODE>,
                    }
                    gpio!(@common_impl $index, $pin, [ $($mux),+ ], $default_mode);
                    gpio!(@irq_impl $index, $pin);
                    impl<MODE> [< PIO $index _ $pin >]<MODE> {
                        #[inline]
                        fn gpio() -> GPIO {
                            unsafe { GPIO::steal() }
                        }

                        #[inline]
                        fn port() -> PORT {
                            unsafe { PORT::steal() }
                        }

                        #[inline]
                        pub const fn pin() -> usize {
                            $pin
                        }
                    }
                    impl<MODE> [< PIO $index _ $pin >]<MODE> {
                        $( gpio!(@pfe_impl $pfe); )?
                        $( gpio!(@pv_impl $pv); )?
                    }
                )+
            }
        }
    };

    (@common_impl $index:expr, $pin:expr, [ $($mux:expr),+ ], $default_mode:ty) => {
        paste::paste! {
            impl<MODE> [< PIO $index _ $pin >]<MODE> {
                pub fn into_push_pull_output(self) -> [< PIO $index _ $pin >]<Output<PushPull>> {
                    Self::gpio().pddr().modify(|r, w| unsafe { w.bits(r.bits() | (1 << $pin)) });
                    Self::port().pcr(Self::pin()).write(|w| unsafe { w.ibe().set_bit().ode().clear_bit().pe().clear_bit().mux().bits(0) });
                    [< PIO $index _ $pin >] { _mode: PhantomData }
                }
                pub fn into_open_drain_output(self) -> [< PIO $index _ $pin >]<Output<OpenDrain>> {
                    Self::gpio().pddr().modify(|r, w| unsafe { w.bits(r.bits() | (1 << $pin)) });
                    Self::port().pcr(Self::pin()).write(|w| unsafe { w.ibe().set_bit().ode().set_bit().pe().clear_bit().mux().bits(0) });
                    [< PIO $index _ $pin >] { _mode: PhantomData }
                }
                pub fn into_pull_up_input(self) -> [< PIO $index _ $pin >]<Input<PullUp>> {
                    Self::gpio().pddr().modify(|r, w| unsafe { w.bits(r.bits() & !(1 << $pin)) });
                    Self::port().pcr(Self::pin()).write(|w| unsafe { w.ibe().set_bit().ode().clear_bit().pe().set_bit().ps().set_bit().mux().bits(0) });
                    [< PIO $index _ $pin >] { _mode: PhantomData }
                }
                pub fn into_pull_down_input(self) -> [< PIO $index _ $pin >]<Input<PullDown>> {
                    Self::gpio().pddr().modify(|r, w| unsafe { w.bits(r.bits() & !(1 << $pin)) });
                    Self::port().pcr(Self::pin()).write(|w| unsafe { w.ibe().set_bit().ode().clear_bit().pe().set_bit().ps().clear_bit().mux().bits(0) });
                    [< PIO $index _ $pin >] { _mode: PhantomData }
                }
                pub fn into_floating_input(self) -> [< PIO $index _ $pin >]<Input<Floating>> {
                    Self::gpio().pddr().modify(|r, w| unsafe { w.bits(r.bits() & !(1 << $pin)) });
                    Self::port().pcr(Self::pin()).write(|w| unsafe { w.ibe().set_bit().ode().clear_bit().pe().clear_bit().mux().bits(0) });
                    [< PIO $index _ $pin >] { _mode: PhantomData }
                }
                pub fn into_analog(self) -> [< PIO $index _ $pin >]<Analog> {
                    Self::port().pcr(Self::pin()).write(|w| unsafe { w.ibe().clear_bit().mux().bits(0) });
                    [< PIO $index _ $pin >] { _mode: PhantomData }
                }
            }
            impl<MODE> [< PIO $index _ $pin >]<Input<MODE>> {
                pub fn into_mux<const MUX: u8>(self) -> [< PIO $index _ $pin >]<Muxed<MODE, MUX>> {
                    struct CHECK<const M: u8>;
                    impl<const M: u8> CHECK<M> {
                        const MUX_VALUE: u8 = match M {
                            $($mux => $mux,)+
                            _ => panic!(concat!("MUX ", stringify!(MUX), " value is not available, available " , stringify!($($mux)*) )),
                        };
                    }

                    Self::port().pcr(Self::pin()).modify(|_, w| unsafe { w.mux().bits(CHECK::<MUX>::MUX_VALUE) });
                    [< PIO $index _ $pin >] { _mode: PhantomData }
                }
                pub fn downgrade(self) -> Pin<Input<MODE>, $index, $pin> {
                    Pin { _mode: PhantomData }
                }
            }
            impl<MODE> [< PIO $index _ $pin >]<Output<MODE>> {
                pub fn into_mux<const MUX: u8>(self) -> [< PIO $index _ $pin >]<Muxed<MODE, MUX>> {
                    struct CHECK<const M: u8>;
                    impl<const M: u8> CHECK<M> {
                        const MUX_VALUE: u8 = match M {
                            $($mux => $mux,)+
                            _ => panic!(concat!("MUX ", stringify!(MUX), " value is not available, available " , stringify!($($mux)*) )),
                        };
                    }

                    Self::port().pcr(Self::pin()).modify(|_, w| unsafe { w.mux().bits(CHECK::<MUX>::MUX_VALUE) });
                    [< PIO $index _ $pin >] { _mode: PhantomData }
                }
                pub fn downgrade(self) -> Pin<Output<MODE>, $index, $pin> {
                    Pin { _mode: PhantomData }
                }
            }
            impl<MODE, const M: u8> [< PIO $index _ $pin >]<Muxed<MODE, M>> {
                pub fn into_mux<const MUX: u8>(self) -> [< PIO $index _ $pin >]<Muxed<MODE, MUX>> {
                    struct CHECK<const M: u8>;
                    impl<const M: u8> CHECK<M> {
                        const MUX_VALUE: u8 = match M {
                            $($mux => $mux,)+
                            _ => panic!(concat!("MUX ", stringify!(MUX), " value is not available, available " , stringify!($($mux)*) )),
                        };
                    }

                    Self::port().pcr(Self::pin()).modify(|_, w| unsafe { w.mux().bits(CHECK::<MUX>::MUX_VALUE) });
                    [< PIO $index _ $pin >] { _mode: PhantomData }
                }

                pub fn pull_up(self) -> [< PIO $index _ $pin >]<Muxed<PullUp, M>> {
                    Self::port().pcr(Self::pin()).modify(|_, w| w.ode().clear_bit().pe().set_bit().ps().set_bit());
                    [< PIO $index _ $pin >] { _mode: PhantomData }
                }
                pub fn pull_down(self) -> [< PIO $index _ $pin >]<Muxed<PullDown, M>> {
                    Self::port().pcr(Self::pin()).modify(|_, w| w.ode().clear_bit().pe().set_bit().ps().clear_bit());
                    [< PIO $index _ $pin >] { _mode: PhantomData }
                }
                pub fn floating(self) -> [< PIO $index _ $pin >]<Muxed<Floating, M>> {
                    Self::port().pcr(Self::pin()).modify(|_, w| w.ode().clear_bit().pe().clear_bit().ps().clear_bit());
                    [< PIO $index _ $pin >] { _mode: PhantomData }
                }
                pub fn open_drain(self) -> [< PIO $index _ $pin >]<Muxed<OpenDrain, M>> {
                    Self::port().pcr(Self::pin()).modify(|_, w| w.ode().set_bit().pe().clear_bit().ps().clear_bit());
                    [< PIO $index _ $pin >] { _mode: PhantomData }
                }
            }

            impl<MODE> ErrorType for [< PIO $index _ $pin >]<MODE> {
                type Error = Infallible;
            }
            impl<MODE> InputPin for [< PIO $index _ $pin >]<Input<MODE>> {
                fn is_high(&mut self) -> Result<bool, Self::Error> {
                    Ok(Self::gpio().pdir().read().bits() >> Self::pin() & 1 == 1)
                }
                fn is_low(&mut self) -> Result<bool, Self::Error> {
                    Ok(!self.is_high()?)
                }
            }
            impl<MODE> OutputPin for [< PIO $index _ $pin >]<Output<MODE>> {
                fn set_low(&mut self) -> Result<(), Self::Error> {
                    Self::gpio().pcor().write(|w| unsafe { w.bits(1 << Self::pin()) });
                    Ok(())
                }
                fn set_high(&mut self) -> Result<(), Self::Error> {
                    Self::gpio().psor().write(|w| unsafe { w.bits(1 << Self::pin()) });
                    Ok(())
                }
            }
            impl<MODE> StatefulOutputPin for [< PIO $index _ $pin >]<Output<MODE>> {
                fn is_set_high(&mut self) -> Result<bool, Self::Error> {
                    Ok(Self::gpio().pdor().read().bits() >> Self::pin() & 1 == 1)
                }
                fn is_set_low(&mut self) -> Result<bool, Self::Error> {
                    Ok(!self.is_set_high()?)
                }
                fn toggle(&mut self) -> Result<(), Self::Error> {
                    Self::gpio().ptor().write(|w| unsafe { w.bits(1 << Self::pin()) });
                    Ok(())
                }
            }
        }
    };

    (@irq_impl $index:expr, $pin:expr) => {
        paste::paste! {
            impl<MODE> [< PIO $index _ $pin >]<Input<MODE>> {
                #[inline]
                pub fn enable_irq(&mut self, source: GPIOInterruptSource, select: GPIOInterruptSelect) {
                    Self::gpio().icr(Self::pin()).write(|w| unsafe { w.isf().clear_bit_by_one().irqs().bit(select.into()).irqc().bits(source as u8) });
                }
                #[inline]
                pub fn disable_irq(&mut self) {
                    Self::gpio().icr(Self::pin()).write(|w| w.isf().clear_bit_by_one());
                }
                #[inline]
                pub fn check_irq(&self) -> bool {
                    Self::gpio().icr(Self::pin()).read().isf().bit_is_set()
                }
                #[inline]
                pub fn check_irq_with_select(&self, select: GPIOInterruptSelect) -> bool {
                    Self::gpio().isfr(select as usize).read().bits() & (1 << Self::pin()) != 0
                }
                #[inline]
                pub fn clear_irq_flag(&mut self) {
                    Self::gpio().icr(Self::pin()).modify(|_r, w| w.isf().clear_bit_by_one());
                }
            }
            impl<MODE> [< PIO $index _ $pin >]<Output<MODE>> {
                #[inline]
                pub fn enable_irq(&mut self, source: GPIOInterruptSource, select: GPIOInterruptSelect) {
                    Self::gpio().icr(Self::pin()).write(|w| unsafe { w.isf().clear_bit_by_one().irqs().bit(select.into()).irqc().bits(source as u8) });
                }
                #[inline]
                pub fn disable_irq(&mut self) {
                    Self::gpio().icr(Self::pin()).write(|w| w.isf().clear_bit_by_one());
                }
                #[inline]
                pub fn check_irq(&self) -> bool {
                    Self::gpio().icr(Self::pin()).read().isf().bit_is_set()
                }
                #[inline]
                pub fn check_irq_with_select(&self, select: GPIOInterruptSelect) -> bool {
                    Self::gpio().isfr(select as usize).read().bits() & (1 << Self::pin()) != 0
                }
                #[inline]
                pub fn clear_irq_flag(&mut self) {
                    Self::gpio().icr(Self::pin()).modify(|_r, w| w.isf().clear_bit_by_one());
                }
            }
            impl<MODE, const MUX: u8> [< PIO $index _ $pin >]<Muxed<MODE, MUX>> {
                #[inline]
                pub fn enable_irq(&mut self, source: GPIOInterruptSource, select: GPIOInterruptSelect) {
                    Self::gpio().icr(Self::pin()).write(|w| unsafe { w.isf().clear_bit_by_one().irqs().bit(select.into()).irqc().bits(source as u8) });
                }
                #[inline]
                pub fn disable_irq(&mut self) {
                    Self::gpio().icr(Self::pin()).write(|w| w.isf().clear_bit_by_one());
                }
                #[inline]
                pub fn check_irq(&self) -> bool {
                    Self::gpio().icr(Self::pin()).read().isf().bit_is_set()
                }
                #[inline]
                pub fn check_irq_with_select(&self, select: GPIOInterruptSelect) -> bool {
                    Self::gpio().isfr(select as usize).read().bits() & (1 << Self::pin()) != 0
                }
                #[inline]
                pub fn clear_irq_flag(&mut self) {
                    Self::gpio().icr(Self::pin()).modify(|_r, w| w.isf().clear_bit_by_one());
                }
            }
        }
    };

    (@pfe_impl true) => {
        pub fn enable_pfe(&mut self) {
            Self::port().pcr(Self::pin()).modify(|_, w| w.pfe().set_bit());
        }
        pub fn disable_pfe(&mut self) {
            Self::port().pcr(Self::pin()).modify(|_, w| w.pfe().clear_bit());
        }
        pub fn is_pfe_enabled(&self) -> bool {
            Self::port().pcr(Self::pin()).read().pfe().bit_is_set()
        }
        pub fn is_pfe_disabled(&self) -> bool {
            !self.is_pfe_enabled()
        }
    };
    (@pfe_impl false) => {};

    (@pv_impl true) => {
        pub fn high_pull_value(&mut self) {
            Self::port().pcr(Self::pin()).modify(|_, w| w.pv().set_bit());
        }
        pub fn low_pull_value(&mut self) {
            Self::port().pcr(Self::pin()).modify(|_, w| w.pv().clear_bit());
        }
        pub fn is_pull_value_high(&self) -> bool {
            Self::port().pcr(Self::pin()).read().pv().bit_is_set()
        }
        pub fn is_pull_value_low(&self) -> bool {
            !self.is_pull_value_high()
        }
    };
    (@pv_impl false) => {};
}
pub use crate::chip::gpio::*;
pub(crate) use gpio;
pub mod all_gpio {
    pub use crate::chip::all_gpio::*;
}
