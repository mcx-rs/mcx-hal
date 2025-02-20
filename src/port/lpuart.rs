//! LPUART pin define.

use crate::{port::Port, private::Sealed};

pub trait Pin: Port {
    type Module;
    type Signal;
}

pub trait Signal: Sealed {}

pub enum TXD {}
pub enum RXD {}

impl Sealed for TXD {}
impl Sealed for RXD {}
impl Signal for TXD {}
impl Signal for RXD {}

macro_rules! lpuart {
    (pin: $pin:ty, module: $module:ident, signal: $signal:ident) => {
        impl crate::port::lpuart::Pin for $pin {
            type Module = crate::port::consts::$module;
            type Signal = crate::port::lpuart::$signal;
        }
    };
}
pub(crate) use lpuart;
