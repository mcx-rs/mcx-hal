//! # NXP MCX Series MCU Rust HAL
//!

#![no_std]

#[cfg_attr(feature = "n947", path = "chip/n947.rs")]
mod chip;

pub use chip::pac;

pub mod clock;
pub mod gpio;
pub mod power;

pub(crate) mod sealed {
    pub trait Sealed {}
}
