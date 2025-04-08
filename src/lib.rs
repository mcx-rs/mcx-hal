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

mod private {
    /// Sealed trait to protect crate traits not implemented by outside crate code.
    pub trait Sealed {}
}

#[cfg(not(feature = "device"))]
compile_error!("Please select one device");
