//! # General Purpose Input Output
//!
//! This driver configurate both PORTN and GPION registers as they always works
//! together.
//!

use crate::pac::{
    GPIO0, GPIO1, GPIO2, GPIO3, GPIO4, GPIO5, PORT0, PORT1, PORT2, PORT3, PORT4, PORT5,
};
use core::{convert::Infallible, marker::PhantomData};

#[derive(Debug, PartialEq, Eq)]
pub enum Level {
    Low,
    High,
}

pub struct Unknown {}

/// Input mode
pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

/// Floating input
pub struct Floating;
/// Pulled down input
pub struct PullDown;
/// Pulled up input
pub struct PullUp;

/// Output mode
pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

/// Push pull output
pub struct PushPull;
/// Open drain output
pub struct OpenDrain;

pub trait Pin {}

pub trait InputPin: Pin {}

pub trait OutputPin: Pin {}

pub struct GPIOPin<MODE, const PORT: u8, const NUM: u8> {
    _mode: PhantomData<MODE>,
}

impl<MODE, const PORT: u8, const NUM: u8> embedded_hal::digital::ErrorType
    for GPIOPin<Input<MODE>, PORT, NUM>
{
    type Error = Infallible;
}

impl<MODE, const PORT: u8, const NUM: u8> embedded_hal::digital::ErrorType
    for GPIOPin<Output<MODE>, PORT, NUM>
{
    type Error = Infallible;
}

impl<MODE, const PORT: u8, const NUM: u8> embedded_hal::digital::InputPin
    for GPIOPin<Input<MODE>, PORT, NUM>
{
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        todo!()
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        todo!()
    }
}

impl<const PORT: u8, const NUM: u8> embedded_hal::digital::InputPin
    for GPIOPin<Output<OpenDrain>, PORT, NUM>
{
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        todo!()
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        todo!()
    }
}

macro_rules! gpio {
    (
        $port_num:literal, [
            $($pin_num: expr, )+
        ]
    ) => {
        paste::paste! {
            mod [< port $port_num >] {
                use super::{
                    [< PORT $port_num >] as PORT,
                    [< GPIO $port_num >] as GPIO,
                    PhantomData, GPIOPin,
                    Unknown,
                    Input, Floating, PullDown, PullUp,
                };

                pub struct Parts {
                    $( pub [< p $pin_num >]: [< P $pin_num >]<Unknown>, )+
                }

                $(
                    pub struct [< P $pin_num >]<MODE> {
                        _mode: PhantomData<MODE>,
                    }

                    impl<MODE> [< P $pin_num >]<MODE> {
                        pub fn into_floating_input(self) -> [< P $pin_num >]<Input<Floating>> {
                            // TODO: configure with PORT and GPIO
                            [< P $pin_num>] { _mode: PhantomData }
                        }

                        pub fn degrade(self) -> GPIOPin<MODE, $port_num, $pin_num> {
                            GPIOPin { _mode: PhantomData }
                        }
                    }
                )+
            }
        }
    };
}

gpio!(0, [1, 2, 3, 4, 5, 6,]);
