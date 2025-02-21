//! Hardware Abstraction Layer for NXP MCX Series MCUs.

#![no_std]

extern crate static_assertions;

pub mod port;

use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature = "device")] {
        pub use mcx_pac as pac;

        pub mod syscon;
    }
}

mod private {
    /// Sealed trait is used to make sure the crate public trait not
    /// implemented by outside crate types.
    pub trait Sealed {}
}
