//! Hardware Abstraction Layer for NXP MCX Series MCUs.

#![no_std]

pub use mcx_pac as pac;

pub mod device;
pub mod gpio;
pub mod lpuart;
pub mod port;
pub mod scg;
pub mod spc;
pub mod syscon;

pub mod consts {
    #[derive(Debug)]
    pub enum Const<const N: u8> {}
    pub trait Unsigned {
        const USIZE: usize;
        fn to_usize() -> usize {
            Self::USIZE
        }
    }
    impl<const N: u8> Unsigned for Const<N> {
        const USIZE: usize = N as usize;
    }
    macro_rules! ux {
        ($($Ux:ident => $N:literal,)+) => {
            $(pub type $Ux = Const<$N>;)+
        };
    }
    ux! {
        U0 => 0, U1 => 1, U2 => 2, U3 => 3, U4 => 4,
        U5 => 5, U6 => 6, U7 => 7, U8 => 8, U9 => 9,
        U10 => 10, U11 => 11, U12 => 12, U13 => 13, U14 => 14,
        U15 => 15, U16 => 16, U17 => 17, U18 => 18, U19 => 19,
        U20 => 20, U21 => 21, U22 => 22, U23 => 23, U24 => 24,
        U25 => 25, U26 => 26, U27 => 27, U28 => 28, U29 => 29,
        U30 => 30, U31 => 31,
    }
}

mod private {
    /// Sealed trait to protect crate traits not implemented by outside crate code.
    pub trait Sealed {}
}

pub mod prelude {
    pub use crate::pac::{self, interrupt};

    pub use crate::gpio::*;

    pub use crate::lpuart::{
        BaudRate, Direction as LpUartDirection, LpUart, LpUartInterrupt, Pins as LpUartPins,
    };

    pub use crate::port::{device::*, Port, PortPin};

    pub use crate::scg::{Config as SCGConfig, FIRC, SCG};

    pub use crate::syscon::*;

    pub use eh1::digital::{InputPin, OutputPin, StatefulOutputPin};
    pub use eio06::*;
}

#[cfg(not(feature = "device"))]
compile_error!("Please select one device");
