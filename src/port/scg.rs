//! SCG pin define.

use crate::{port::Port, private};

pub trait Pin: Port {
    type Module;
    type Signal;
}

pub trait Signal {}
impl Signal for XTAL48M {}
impl Signal for EXTAL48M {}

pub enum XTAL48M {}
pub enum EXTAL48M {}
impl private::Sealed for XTAL48M {}
impl private::Sealed for EXTAL48M {}

pub fn prepare<P: Pin>(p: &mut P) {
    p.analog(true);
}

macro_rules! scg {
    (pin: $pin:ty, module: $module:ident, signal: $signal:ident) => {
        impl crate::port::scg::Pin for $pin {
            type Module = crate::port::consts::$module;
            type Signal = crate::port::scg::$signal;
        }
    };
}
pub(crate) use scg;
