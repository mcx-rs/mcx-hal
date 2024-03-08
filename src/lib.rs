//! # NXP MCX Series MCU Rust HAL
//!

#![no_std]

#[cfg(not(feature = "_device-selected"))]
compile_error!("should select one device");

#[cfg_attr(feature = "n947", path = "chip/n947.rs")]
mod chip;

pub use chip::pac;

pub mod clock;
pub mod gpio;
// pub mod lpuart;
pub mod mailbox;
pub mod power;

pub(crate) mod sealed {
    pub trait Sealed {}
}

#[derive(Clone, Copy, Debug)]
pub enum HalError {
    InvalidConfig,
}

impl sealed::Sealed for HalError {}
