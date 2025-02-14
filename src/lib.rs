//! Hardware Abstraction Layer for NXP MCX Series MCUs.

#![no_std]

pub use mcx_pac as pac;

pub mod port;
pub mod syscon;

pub mod prelude {}

#[derive(Clone, Copy, Debug)]
pub enum HalError {
    Fail,
    ReadOnly,
    OutOfRange,
    InvalidValue,
    Timeout,
    Busy,
}

mod private {
    /// Sealed trait to protect crate traits not implemented by outside crate code.
    pub trait Sealed {}
}
