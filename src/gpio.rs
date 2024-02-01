//! # General Purpose Input / Output
//!
//! ##
//!
//! ## Design
//! We do not use a type trait for example GpioExt here, because NXP's GPIO takes two peripherals:
//! GPION and PORTN, they are always used together.
//! Peripheral GPION is used to read and set GPIO's output and input, and PORTN is used to configure
//! pin's mux and input output mode such as floating and pushpull.

use crate::pac::{
    gpio0::RegisterBlock as GPIORegisterBlock, port0::RegisterBlock as PORTRegisterBlock,
};
use core::marker::PhantomData;

/// Input mode, type state
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Output mode, type state
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Floating input
pub struct Floating;
/// Pulled down input
pub struct PullDown;
/// Pulled up input
pub struct PullUp;
/// Push pull output
pub struct PushPull;

/// Open drain output
pub struct OpenDrain;

pub struct Pin<MODE, const PORT: u8> {
    num: u8,
    _mode: PhantomData<MODE>,
}

impl<MODE, const PORT: u8> Pin<MODE, PORT> {
    fn pin_id(&self) -> u8 {
        self.num
    }

    fn port_id(&self) -> u8 {
        PORT
    }
}

const fn get_port_ptr(port: u8) -> *const PORTRegisterBlock {
    // TODO: use cfg attr to check port number
    match port {
        0 => crate::pac::PORT0::ptr(),
        1 => crate::pac::PORT1::ptr(),
        2 => crate::pac::PORT2::ptr(),
        3 => crate::pac::PORT3::ptr(),
        4 => crate::pac::PORT4::ptr(),
        5 => crate::pac::PORT5::ptr(),
        _ => unreachable!(),
    }
}

const fn get_gpio_ptr(gpio: u8) -> *const GPIORegisterBlock {
    // TODO: use cfg attr to check gpio number
    match gpio {
        0 => crate::pac::GPIO0::ptr(),
        1 => crate::pac::GPIO1::ptr(),
        2 => crate::pac::GPIO2::ptr(),
        3 => crate::pac::GPIO3::ptr(),
        4 => crate::pac::GPIO4::ptr(),
        5 => crate::pac::GPIO5::ptr(),
        _ => unreachable!(),
    }
}

macro_rules! gpio {
    (
        // $port_num: literal, $gpio_num: literal, [
        //     [$($pin_num: literal, [$($mux: literal),*] $(, $default_mode: ty)?, )+]
        // ]
        $port_num:literal, $gpio_num:literal, [
            $($pin_num:literal: [$($mux:literal), *], $MODE:ty,)+
        ]
    ) => {
        paste::paste! {
            pub mod [< port $port_num >] {
                use core::marker::PhantomData;
                use $crate::pac::{[< GPIO $gpio_num >] as GPIO, [< PORT $port_num >] as PORT};
                use super::{
                    Input, Output, Floating, PullDown, PullUp, PushPull, OpenDrain,
                };
                use super::Pin;

                pub fn split(gpio: GPIO, port: PORT) -> Parts {
                    Parts::new(gpio, port)
                }

                pub struct Parts {
                    $(
                        pub [< pio $gpio_num _ $pin_num >]: [< PIO $gpio_num _ $pin_num >]<$MODE>,
                    )+
                }

                impl Parts {
                    pub fn new(_gpio: GPIO, _port: PORT) -> Self {
                        Self {
                            $(
                                [< pio $gpio_num _ $pin_num >]: [< PIO $gpio_num _ $pin_num >] {
                                    _mode: PhantomData,
                                },
                            )+
                        }
                    }
                }

                $(
                    pub struct [< PIO $gpio_num _ $pin_num >]<MODE> {
                        _mode: PhantomData<MODE>,
                    }

                    impl<MODE> [< PIO $gpio_num _ $pin_num >]<MODE> {
                        pub fn downgrade(self) -> Pin<MODE, $port_num> {
                            Pin {
                                num: $pin_num,
                                _mode: PhantomData,
                            }
                        }
                    }
                )+
            }
        }
    };
}

gpio!(0, 0, [
    0: [0, 1, 2], Input<Floating>,
]);
