#![no_std]

#[cfg(feature = "n947")]
pub use mcxn947_pac as pac;

use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

pub mod gpio;

pub(crate) mod sealed {
    pub trait Sealed {}
}
