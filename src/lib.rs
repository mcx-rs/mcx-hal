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

macro_rules! is_trait {
    ($name:ty, $trait_name:path) => {{
        trait __InnerMarkerTrait {
            fn __is_trait_inner_method() -> bool {
                false
            }
        }
        struct __TraitTest<T>(T);
        impl<T: $trait_name> __TraitTest<T> {
            fn __is_trait_inner_method() -> bool {
                true
            }
        }
        impl<T> __InnerMarkerTrait for __TraitTest<T> {}
        __TraitTest::<$name>::__is_trait_inner_method()
    }};
}
pub(crate) use is_trait;
