//! PORT configuration and PIN constrain for NXP MCX Series MCUs.

use crate::{
    consts::{Const, Unsigned},
    pac::{
        common::{Reg, RW},
        port::{regs::PCR, Instance, LEN},
    },
    private::Sealed,
};

pub mod lpuart;
pub mod scg;

pub(crate) use lpuart::lpuart;
pub(crate) use scg::scg;

/// Port trait for MCX N & A PORT peripheral.
pub trait Port: Sealed {
    type PORT: Unsigned;
    type PIN: Unsigned;

    fn mux(&self) -> u8;
    fn set_mux(&mut self, v: u8);

    // fn port(&self) -> u8;
    // fn pin(&self) -> u8;

    fn floating(&mut self);
    fn pull(&mut self, up: bool);
    fn open_drain(&mut self, enable: bool);
    fn analog(&mut self, enable: bool);
}

pub struct PortPin<const PORT: u8, const PIN: u8>;
unsafe impl<const PORT: u8, const PIN: u8> Send for PortPin<PORT, PIN> {}
unsafe impl<const PORT: u8, const PIN: u8> Sync for PortPin<PORT, PIN> {}
impl<const PORT: u8, const PIN: u8> Sealed for PortPin<PORT, PIN> {}
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
    type PORT = Const<PORT>;
    type PIN = Const<PIN>;

    #[inline(always)]
    fn mux(&self) -> u8 {
        self.pcr().read().MUX()
    }
    #[inline(always)]
    fn set_mux(&mut self, v: u8) {
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

#[cfg(feature = "device")]
pub mod device {
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
#[cfg(feature = "device")]
pub use device::*;
