//! Pin configuration for NXP MCX series MCUs.

use crate::{
    pac::{
        common::{Reg, RW},
        port::{regs::PCR, Instance},
    },
    private::Sealed,
};

pub mod lpuart;
pub mod scg;

mod device {
    use cfg_if::cfg_if;
    cfg_if! {
        if #[cfg(feature = "mcxa0")] {
            mod a0;
            pub use a0::*;
        } else if #[cfg(feature = "mcxa1")] {
            mod a1;
            pub use a1::*;
        } else if #[cfg(feature = "mcxa2")] {
            mod a2;
            pub use a2::*;
        }
    }
}
pub use device::*;

/// Port trait for MCU pin.
///
/// # Safety
/// This trait should only be implementd on PCR based PORT peripherals.
pub unsafe trait Port: Sealed {
    type PCR: Copy;

    /// Get port index.
    fn port(&self) -> u8;
    /// Get pin index.
    fn pin(&self) -> u8;
    /// Get current mux.
    fn mux(&self) -> u8;
    /// Set current mux.
    fn set_mux(&mut self, mux: u8);

    /// Get PCR register to modify.
    ///
    /// # Safety
    /// Any modification is dangerous, will break current pin function.
    unsafe fn pcr(&self) -> Reg<Self::PCR, RW>;

    /// Disconnect internal pull-up or pull-down registers.
    fn floating(&mut self);
    /// Select internal pull-up or pull-down register.
    fn pull(&mut self, up: bool);
    /// Enable open drain output.
    fn open_drain(&mut self, enable: bool);
    /// Disable input buffer, ready for analog functions.
    fn analog(&mut self, enable: bool);
}

/// A MCX MCU pin.
pub struct PortPin<const PORT: u8, const PIN: u8>;
unsafe impl<const PORT: u8, const PIN: u8> Send for PortPin<PORT, PIN> {}
unsafe impl<const PORT: u8, const PIN: u8> Sync for PortPin<PORT, PIN> {}
impl<const PORT: u8, const PIN: u8> PortPin<PORT, PIN> {
    const CHECK: () = assert!(PORT < crate::pac::port::ADDRESSES.len() as u8);

    pub(crate) const unsafe fn new() -> Self {
        #![allow(clippy::let_unit_value)]
        let _check = Self::CHECK;
        Self {}
    }
}
impl<const PORT: u8, const PIN: u8> Sealed for PortPin<PORT, PIN> {}
unsafe impl<const PORT: u8, const PIN: u8> Port for PortPin<PORT, PIN> {
    type PCR = PCR;

    #[inline(always)]
    fn port(&self) -> u8 {
        PORT
    }
    #[inline(always)]
    fn pin(&self) -> u8 {
        PIN
    }
    #[inline(always)]
    fn mux(&self) -> u8 {
        unsafe { self.pcr().read().MUX() }
    }
    #[inline(always)]
    fn set_mux(&mut self, mux: u8) {
        unsafe {
            self.pcr().modify(|r| r.set_MUX(mux));
        }
    }

    #[inline(always)]
    unsafe fn pcr(&self) -> Reg<Self::PCR, RW> {
        Instance::<PORT>::instance().regs().PCR(PIN as usize)
    }
    #[inline(always)]
    fn floating(&mut self) {
        unsafe { self.pcr().modify(|r| r.set_PE(false)) }
    }
    #[inline(always)]
    fn pull(&mut self, up: bool) {
        unsafe {
            self.pcr().modify(|r| {
                r.set_PE(true);
                r.set_PS(up);
            })
        }
    }
    #[inline(always)]
    fn open_drain(&mut self, enable: bool) {
        unsafe { self.pcr().modify(|r| r.set_ODE(enable)) }
    }
    #[inline(always)]
    fn analog(&mut self, enable: bool) {
        unsafe { self.pcr().modify(|r| r.set_IBE(!enable)) }
    }
}

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
        U30 => 30, U31 => 31, U32 => 32, U33 => 33, U34 => 34,
        U35 => 35, U36 => 36, U37 => 37, U38 => 38, U39 => 39,
        U40 => 40, U41 => 41,
    }
}
