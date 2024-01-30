#![no_std]

#[cfg(feature = "n947")]
use mcxn947_pac as pac;

use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

pub(crate) mod sealed {
    pub trait Sealed {}
}

///
pub trait Peripheral: Sized + sealed::Sealed {
    type P;

    unsafe fn clone_unchecked(&mut self) -> Self::P;

    #[inline]
    fn into_ref<'a>(mut self) -> PeripheralRef<'a, Self::P>
    where
        Self: 'a,
    {
        PeripheralRef::new(unsafe { self.clone_unchecked() })
    }
}

///
pub struct PeripheralRef<'a, T> {
    inner: T,
    _lifetime: PhantomData<&'a mut T>,
}

impl<T> sealed::Sealed for &mut T where T: sealed::Sealed {}

impl<T> Peripheral for &mut T
where
    T: Peripheral<P = T>,
{
    type P = T;

    unsafe fn clone_unchecked(&mut self) -> Self::P {
        T::clone_unchecked(self)
    }
}

impl<'a, T> PeripheralRef<'a, T> {
    #[inline]
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            _lifetime: PhantomData,
        }
    }

    pub unsafe fn clone_unchecked(&mut self) -> PeripheralRef<'a, T>
    where
        T: Peripheral<P = T>,
    {
        PeripheralRef::new(self.inner.clone_unchecked())
    }

    pub fn reborrow(&mut self) -> PeripheralRef<'_, T>
    where
        T: Peripheral<P = T>,
    {
        PeripheralRef::new(unsafe { self.inner.clone_unchecked() })
    }

    #[inline]
    pub fn map_into<U>(self) -> PeripheralRef<'a, U>
    where
        T: Into<U>,
    {
        PeripheralRef {
            inner: self.inner.into(),
            _lifetime: PhantomData,
        }
    }
}

impl<'a, T> Deref for PeripheralRef<'a, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a, T> DerefMut for PeripheralRef<'a, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

mod gpio;
