// //! # General Purpose Input / Output
// //!
// //! ##
// //!
// //! ## Design
// //! We do not use a type trait for example GpioExt here, because NXP's GPIO takes two peripherals:
// //! GPION and PORTN, they are always used together.
// //! Peripheral GPION is used to read and set GPIO's output and input, and PORTN is used to configure
// //! pin's mux and input output mode such as floating and pushpull.

// use crate::pac::gpio0::RegisterBlock as GPIORegisterBlock;
// use core::{convert::Infallible, marker::PhantomData};
// use embedded_hal::digital::{ErrorType, InputPin, OutputPin, StatefulOutputPin};

// /// Disabled mode, type state
// /// default mode
// pub struct Disabled;

// /// Input mode, type state
// pub struct Input<MODE> {
//     _mode: PhantomData<MODE>,
// }

// /// Output mode, type state
// pub struct Output<MODE> {
//     _mode: PhantomData<MODE>,
// }

// pub struct Muxed<MODE, const MUX: u8> {
//     _mode: PhantomData<MODE>,
// }

// /// Floating input
// pub struct Floating;
// /// Pulled down input
// pub struct PullDown;
// /// Pulled up input
// pub struct PullUp;
// /// Push pull output
// pub struct PushPull;
// /// Open drain output
// pub struct OpenDrain;
// /// Analog
// pub struct Analog;

// pub struct Pin<MODE, const PORT: u8> {
//     pub(crate) num: u8,
//     pub(crate) _mode: PhantomData<MODE>,
// }

// impl<MODE, const PORT: u8> Pin<MODE, PORT> {
//     /// get the pin number
//     pub fn pin_id(&self) -> u8 {
//         self.num
//     }

//     /// get the port number
//     pub fn port_id(&self) -> u8 {
//         PORT
//     }
// }

// impl<MODE, const PORT: u8> ErrorType for Pin<MODE, PORT> {
//     type Error = Infallible;
// }

// impl<MODE, const PORT: u8> OutputPin for Pin<Output<MODE>, PORT> {
//     fn set_low(&mut self) -> Result<(), Self::Error> {
//         let gpio = get_gpio_ptr(PORT);
//         unsafe { (*gpio).pcor().write(|w| w.bits(1 << self.pin_id())) };
//         Ok(())
//     }

//     fn set_high(&mut self) -> Result<(), Self::Error> {
//         let gpio = get_gpio_ptr(PORT);
//         unsafe { (*gpio).psor().write(|w| w.bits(1 << self.pin_id())) };
//         Ok(())
//     }
// }

// impl<MODE, const PORT: u8> StatefulOutputPin for Pin<Output<MODE>, PORT> {
//     fn is_set_high(&mut self) -> Result<bool, Self::Error> {
//         let gpio = get_gpio_ptr(PORT);
//         Ok(unsafe { (*gpio).pdor().read().bits() & (1 << self.pin_id()) != 0 })
//     }

//     fn is_set_low(&mut self) -> Result<bool, Self::Error> {
//         Ok(!self.is_set_high()?)
//     }
// }

// impl<MODE, const PORT: u8> InputPin for Pin<Input<MODE>, PORT> {
//     fn is_high(&mut self) -> Result<bool, Self::Error> {
//         let gpio = get_gpio_ptr(PORT);
//         Ok(unsafe { (*gpio).pdir().read().bits() & (1 << self.pin_id()) != 0 })
//     }

//     fn is_low(&mut self) -> Result<bool, Self::Error> {
//         Ok(!self.is_high()?)
//     }
// }

// pub(crate) const fn get_gpio_ptr(gpio: u8) -> *const GPIORegisterBlock {
//     // TODO: use cfg attr to check gpio number
//     match gpio {
//         0 => crate::pac::GPIO0::ptr(),
//         1 => crate::pac::GPIO1::ptr(),
//         2 => crate::pac::GPIO2::ptr(),
//         3 => crate::pac::GPIO3::ptr(),
//         4 => crate::pac::GPIO4::ptr(),
//         5 => crate::pac::GPIO5::ptr(),
//         _ => unreachable!(),
//     }
// }

// macro_rules! gpio {
//     (
//         $port_num:literal, $gpio_num:literal, [
//             $($pin_num:literal: [$($mux:literal), *], $MODE:ty,)+
//         ]
//     ) => {
//         paste::paste! {
//             pub mod [< gpio $gpio_num >] {
//                 use core::marker::PhantomData;
//                 use $crate::pac::{[< GPIO $gpio_num >] as GPIO, [< PORT $port_num >] as PORT};
//                 use $crate::gpio::{
//                     Input, Output, Floating, PullDown, PullUp, PushPull, OpenDrain, Disabled, Muxed,
//                 };
//                 use $crate::gpio::Pin;
//                 use $crate::clock::PeripheralClocks;
//                 use $crate::sealed::Sealed;

//                 pub fn split(gpio: GPIO, port: PORT) -> Parts {
//                     Parts::new(gpio, port)
//                 }

//                 pub struct Parts {
//                     $(
//                         pub [< pio $gpio_num _ $pin_num >]: [< PIO $gpio_num _ $pin_num >]<$MODE>,
//                     )+
//                 }

//                 impl Parts {
//                     pub fn new(_gpio: GPIO, _port: PORT) -> Self {
//                         // TODO: find a way to properly enable PORT and GPIO clocks
//                         PeripheralClocks::[< GPIO $gpio_num >]::enable();
//                         PeripheralClocks::[< PORT $port_num >]::enable();
//                         Self {
//                             $(
//                                 [< pio $gpio_num _ $pin_num >]: [< PIO $gpio_num _ $pin_num >] {
//                                     _mode: PhantomData,
//                                 },
//                             )+
//                         }
//                     }
//                 }

//                 $(
//                     /// GPIO pin
//                     pub struct [< PIO $gpio_num _ $pin_num >]<MODE> {
//                         _mode: PhantomData<MODE>,
//                     }

//                     impl<MODE> Sealed for [< PIO $gpio_num _ $pin_num >]<MODE> {}

//                     impl<MODE> [< PIO $gpio_num _ $pin_num >]<MODE> {
//                         /// Configure the pin to floating input
//                         pub fn into_floating_input(self) -> [< PIO $gpio_num _ $pin_num >]<Input<Floating>> {
//                             let gpio = unsafe { $crate::pac::[< GPIO $gpio_num >]::steal() };
//                             let port = unsafe { $crate::pac::[< PORT $port_num >]::steal() };

//                             gpio.pddr().modify(|_, w| w.[< pdd $pin_num >]().clear_bit());
//                             port.pcr($pin_num).modify(|_, w| unsafe { w.ibe().set_bit().pe().clear_bit().mux().bits(0) });

//                             [< PIO $gpio_num _ $pin_num >] { _mode: PhantomData }
//                         }

//                         /// Configure the pin to pull-down input
//                         pub fn into_pull_down_input(self) -> [< PIO $gpio_num _ $pin_num >]<Input<PullDown>> {
//                             let gpio = unsafe { $crate::pac::[< GPIO $gpio_num >]::steal() };
//                             let port = unsafe { $crate::pac::[< PORT $port_num >]::steal() };

//                             gpio.pddr().modify(|_, w| w.[< pdd $pin_num >]().clear_bit());
//                             port.pcr($pin_num).modify(|_, w| unsafe { w.ibe().set_bit().pe().set_bit().ps().clear_bit().ode().clear_bit().mux().bits(0) });

//                             [< PIO $gpio_num _ $pin_num >] { _mode: PhantomData }
//                         }

//                         /// Configure the pin to pull-up input
//                         pub fn into_pull_up_input(self) -> [< PIO $gpio_num _ $pin_num >]<Input<PullUp>> {
//                             let gpio = unsafe { $crate::pac::[< GPIO $gpio_num >]::steal() };
//                             let port = unsafe { $crate::pac::[< PORT $port_num >]::steal() };

//                             gpio.pddr().modify(|_, w| w.[< pdd $pin_num >]().clear_bit());
//                             port.pcr($pin_num).modify(|_, w| unsafe { w.ibe().set_bit().pe().set_bit().ps().set_bit().ode().clear_bit().mux().bits(0) });

//                             [< PIO $gpio_num _ $pin_num >] { _mode: PhantomData }
//                         }

//                         /// Configure the pin to open-drain output
//                         pub fn into_open_drain_output(self) -> [< PIO $gpio_num _ $pin_num >]<Output<OpenDrain>> {
//                             let gpio = unsafe { $crate::pac::[< GPIO $gpio_num >]::steal() };
//                             let port = unsafe { $crate::pac::[< PORT $port_num >]::steal() };

//                             gpio.pddr().modify(|_, w| w.[< pdd $pin_num >]().set_bit());
//                             port.pcr($pin_num).modify(|_, w| unsafe { w.ibe().set_bit().pe().clear_bit().ode().set_bit().mux().bits(0) });

//                             [< PIO $gpio_num _ $pin_num >] { _mode: PhantomData }
//                         }

//                         /// Configure the pin to push-pull output
//                         pub fn into_push_pull_output(self) -> [< PIO $gpio_num _ $pin_num >]<Output<PushPull>> {
//                             let gpio = unsafe { $crate::pac::[< GPIO $gpio_num >]::steal() };
//                             let port = unsafe { $crate::pac::[< PORT $port_num >]::steal() };

//                             gpio.pddr().modify(|_, w| w.[< pdd $pin_num >]().set_bit());
//                             port.pcr($pin_num).modify(|_, w| unsafe { w.ibe().set_bit().pe().clear_bit().ode().clear_bit().mux().bits(0) });

//                             [< PIO $gpio_num _ $pin_num >] { _mode: PhantomData }
//                         }

//                         /// Configure the pin to alternate function
//                         pub fn into_mux<const MUX: u8>(self) -> [< PIO $gpio_num _ $pin_num >]<Muxed<MODE, MUX>> {
//                             struct CHECK<const M: u8>;
//                             impl<const M: u8> CHECK<M> {
//                                 const MUX_VALUE: u8 = match M {
//                                     $($mux => $mux,)+
//                                     // _ => panic!("MUX value is not available"),
//                                     _ => panic!(concat!("MUX ", stringify!(MUX), " value is not available, available " , stringify!($($mux)*) )),
//                                 };
//                             }

//                             let port = unsafe { $crate::pac::[< PORT $port_num >]::steal() };
//                             port.pcr($pin_num).modify(|_, w| unsafe { w.mux().bits(CHECK::<MUX>::MUX_VALUE) });

//                             [< PIO $gpio_num _ $pin_num >] { _mode: PhantomData }
//                         }
//                     }

//                     impl<MODE> [< PIO $gpio_num _ $pin_num >]<Output<MODE>> {
//                         pub fn downgrade(self) -> Pin<Output<MODE>, $port_num> {
//                             Pin {
//                                 num: $pin_num,
//                                 _mode: self._mode,
//                             }
//                         }
//                     }
//                     impl<MODE> [< PIO $gpio_num _ $pin_num >]<Input<MODE>> {
//                         pub fn downgrade(self) -> Pin<Input<MODE>, $port_num> {
//                             Pin {
//                                 num: $pin_num,
//                                 _mode: self._mode,
//                             }
//                         }
//                     }

//                     impl Into<[< PIO $gpio_num _ $pin_num >]<Input<Floating>>> for [< PIO $gpio_num _ $pin_num >]<Disabled> {
//                         fn into(self) -> [< PIO $gpio_num _ $pin_num >]<Input<Floating>> {
//                             self.into_floating_input()
//                         }
//                     }
//                     impl Into<[< PIO $gpio_num _ $pin_num >]<Input<PullDown>>> for [< PIO $gpio_num _ $pin_num >]<Disabled> {
//                         fn into(self) -> [< PIO $gpio_num _ $pin_num >]<Input<PullDown>> {
//                             self.into_pull_down_input()
//                         }
//                     }
//                     impl Into<[< PIO $gpio_num _ $pin_num >]<Input<PullUp>>> for [< PIO $gpio_num _ $pin_num >]<Disabled> {
//                         fn into(self) -> [< PIO $gpio_num _ $pin_num >]<Input<PullUp>> {
//                             self.into_pull_up_input()
//                         }
//                     }
//                     impl Into<[< PIO $gpio_num _ $pin_num >]<Output<OpenDrain>>> for [< PIO $gpio_num _ $pin_num >]<Disabled> {
//                         fn into(self) -> [< PIO $gpio_num _ $pin_num >]<Output<OpenDrain>> {
//                             self.into_open_drain_output()
//                         }
//                     }
//                     impl Into<[< PIO $gpio_num _ $pin_num >]<Output<PushPull>>> for [< PIO $gpio_num _ $pin_num >]<Disabled> {
//                         fn into(self) -> [< PIO $gpio_num _ $pin_num >]<Output<PushPull>> {
//                             self.into_push_pull_output()
//                         }
//                     }
//                 )+
//             }
//         }
//     };
// }
// pub(crate) use gpio;

// pub use crate::chip::gpio::*;
// pub mod all_gpio {
//     pub use crate::chip::all_gpio::*;
// }

//! # General Purpose Input/Output
//!

use core::marker::PhantomData;

/// Type state Muxed
pub struct Muxed<MODE, const MUX: u8>(PhantomData<MODE>);
/// Type state DigitalInput
pub struct DigitalInput<MODE>(PhantomData<MODE>);
/// Type state DigitalOutput
pub struct DigitalOutput<MODE>(PhantomData<MODE>);
/// Type state Analog
pub struct Analog;

pub struct Floating;
pub struct PullUp;
pub struct PullDown;
pub struct PushPull;
pub struct OpenDrain;

pub struct Pin<MODE, const PORT: u8, const PIN: u8> {
    _mode: PhantomData<MODE>,
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

pub(crate) mod pin_trait {
    pub trait SRE {}
    pub trait PV {}
}
mod mode_trait {}

macro_rules! gpio {
    // syntax:
    // gpio!(index: 0,
    //     [pin: 0, [0, 1, 2, ...], Input<Floating>: has_sre, has_pv],
    // )
    (
        index: $index:expr,
            $( [pin: $pin:expr, [ $($mux:expr),+ ], $default_mode:ty $(, pfe: $pfe:expr)? $(, pv: $pv:expr)?]),+
    ) => {
        paste::paste! {
            pub mod [< gpio $index >] {
                use core::marker::PhantomData;
                use $crate::pac::{[< GPIO $index >] as GPIO, [< PORT $index >] as PORT};
                use $crate::clock::PeripheralClocks;
                use $crate::gpio::{Muxed, DigitalInput, DigitalOutput, Analog, Floating, PullUp, PullDown, PushPull, OpenDrain};

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

                    impl<MODE> [< PIO $index _ $pin >]<MODE> {
                        #[inline]
                        fn gpio() -> GPIO {
                            return unsafe { GPIO::steal() };
                        }

                        #[inline]
                        fn port() -> PORT {
                            return unsafe { PORT::steal() };
                        }

                        #[inline]
                        pub const fn pin() -> usize {
                            return $pin;
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
                pub fn into_push_pull_output(self) -> [< PIO $index _ $pin >]<DigitalOutput<PushPull>> {
                    Self::gpio().pddr().modify(|r, w| unsafe { w.bits(r.bits() | (1 << $pin)) });
                    Self::port().pcr(Self::pin()).write(|w| unsafe { w.ibe().set_bit().ode().clear_bit().pe().clear_bit().mux().bits(0) });
                    [< PIO $index _ $pin >] { _mode: PhantomData }
                }
                pub fn into_open_drain_output(self) -> [< PIO $index _ $pin >]<DigitalOutput<OpenDrain>> {
                    Self::gpio().pddr().modify(|r, w| unsafe { w.bits(r.bits() | (1 << $pin)) });
                    Self::port().pcr(Self::pin()).write(|w| unsafe { w.ibe().set_bit().ode().set_bit().pe().clear_bit().mux().bits(0) });
                    [< PIO $index _ $pin >] { _mode: PhantomData }
                }
                pub fn into_pull_up_input(self) -> [< PIO $index _ $pin >]<DigitalInput<PullUp>> {
                    Self::gpio().pddr().modify(|r, w| unsafe { w.bits(r.bits() & !(1 << $pin)) });
                    Self::port().pcr(Self::pin()).write(|w| unsafe { w.ibe().set_bit().ode().clear_bit().pe().set_bit().ps().set_bit().mux().bits(0) });
                    [< PIO $index _ $pin >] { _mode: PhantomData }
                }
                pub fn into_pull_down_input(self) -> [< PIO $index _ $pin >]<DigitalInput<PullDown>> {
                    Self::gpio().pddr().modify(|r, w| unsafe { w.bits(r.bits() & !(1 << $pin)) });
                    Self::port().pcr(Self::pin()).write(|w| unsafe { w.ibe().set_bit().ode().clear_bit().pe().set_bit().ps().clear_bit().mux().bits(0) });
                    [< PIO $index _ $pin >] { _mode: PhantomData }
                }
                pub fn into_floating_input(self) -> [< PIO $index _ $pin >]<DigitalInput<Floating>> {
                    Self::gpio().pddr().modify(|r, w| unsafe { w.bits(r.bits() & !(1 << $pin)) });
                    Self::port().pcr(Self::pin()).write(|w| unsafe { w.ibe().set_bit().ode().clear_bit().pe().clear_bit().mux().bits(0) });
                    [< PIO $index _ $pin >] { _mode: PhantomData }
                }
                pub fn into_analog(self) -> [< PIO $index _ $pin >]<Analog> {
                    Self::port().pcr(Self::pin()).write(|w| unsafe { w.ibe().clear_bit().mux().bits(0) });
                    [< PIO $index _ $pin >] { _mode: PhantomData }
                }
            }
            impl<MODE> [< PIO $index _ $pin >]<DigitalInput<MODE>> {
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
            }
            impl<MODE> [< PIO $index _ $pin >]<DigitalOutput<MODE>> {
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
