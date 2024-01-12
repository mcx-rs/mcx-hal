#![no_std]

#[cfg(feature = "n947")]
pub use mcxn947_pac as pac;

pub mod clock;
