//! PORT configuration and PIN constrain for NXP MCX Series MCUs.

use crate::private::Sealed;

pub mod lpuart;
pub mod scg;

pub(crate) use lpuart::lpuart;
pub(crate) use scg::scg;

/// Port trait for MCX N & A PORT peripheral.
pub trait Port: Sealed {
    fn mux(&self) -> u8;
    fn set_mux(&self, v: u8);

    fn floating(&mut self);
    fn pull(&mut self, up: bool);
    fn open_drain(&mut self, enable: bool);
    fn analog(&mut self, enable: bool);
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
        U30 => 30, U31 => 31,
    }
}

#[cfg(feature = "device")]
mod device {
    use crate::{
        pac::{
            common::{Reg, RW},
            port::{regs::PCR, Instance, LEN},
        },
        port::Port,
        private,
    };

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

    pub struct PortPin<const PORT: u8, const PIN: u8>;
    unsafe impl<const PORT: u8, const PIN: u8> Send for PortPin<PORT, PIN> {}
    unsafe impl<const PORT: u8, const PIN: u8> Sync for PortPin<PORT, PIN> {}
    impl<const PORT: u8, const PIN: u8> private::Sealed for PortPin<PORT, PIN> {}
    impl<const PORT: u8, const PIN: u8> PortPin<PORT, PIN> {
        pub const unsafe fn new() -> Self {
            const { assert!(PORT < LEN as u8) }
            const { assert!(PIN < 32) }
            Self {}
        }

        fn pcr(&self) -> Reg<PCR, RW> {
            unsafe { Instance::<PORT>::instance().regs().PCR(PIN as usize) }
        }
    }
    impl<const PORT: u8, const PIN: u8> Port for PortPin<PORT, PIN> {
        #[inline(always)]
        fn mux(&self) -> u8 {
            self.pcr().read().MUX()
        }
        #[inline(always)]
        fn set_mux(&self, v: u8) {
            self.pcr().modify(|r| r.set_MUX(v));
        }
        #[inline(always)]
        fn floating(&mut self) {
            self.pcr().modify(|r| r.set_PE(false));
        }
        #[inline(always)]
        fn pull(&mut self, up: bool) {
            self.pcr().modify(|r| {
                r.set_PE(true);
                r.set_PS(up);
            })
        }
        #[inline(always)]
        fn open_drain(&mut self, enable: bool) {
            self.pcr().modify(|r| r.set_ODE(enable));
        }
        #[inline(always)]
        fn analog(&mut self, enable: bool) {
            self.pcr().modify(|r| r.set_IBE(!enable));
        }
    }
}
#[cfg(feature = "device")]
pub use device::*;
