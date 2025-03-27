//! LPUART pin define

use crate::{port::Port, private};

pub trait Pin: Port {
    type Module;
    type Signal;
    const MUX: u8;
}

pub trait Signal {}
impl Signal for TXD {}
impl Signal for RXD {}

pub enum TXD {}
pub enum RXD {}
impl private::Sealed for TXD {}
impl private::Sealed for RXD {}

pub fn prepare<P: Pin>(p: &mut P) {
    p.analog(false);
    p.set_mux(P::MUX);
}

macro_rules! lpuart {
    (pin: $pin:ty, module: $module:ident, signal: $signal:ident, mux: $mux:expr) => {
        impl crate::port::lpuart::Pin for $pin {
            type Module = crate::port::consts::$module;
            type Signal = crate::port::lpuart::$signal;
            const MUX: u8 = $mux;
        }
    };
}
pub(crate) use lpuart;
