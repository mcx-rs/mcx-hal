// //! SCG pin define.

// use crate::{port::Port, private::Sealed};

// use super::PortPin;

// /// SCG Pin.
// pub trait Pin: Port {
//     type Module;
//     type Signal;
// }

// /// SCG Pin signal.
// ///
// /// See `EXTAL` and `XTAL`
// pub trait Signal: Sealed {}

// /// EXTAL Signal
// pub enum EXTAL {}
// /// XTAL Signal
// pub enum XTAL {}

// impl Sealed for EXTAL {}
// impl Signal for EXTAL {}
// impl Sealed for XTAL {}
// impl Signal for XTAL {}

// pub fn prepare<P: Pin>(pin: &mut P) {
//     pin.analog(true);
// }

// macro_rules! scg {
//     (pin: $pin:ty, module: $module:ident, signal: $signal:ident) => {
//         impl crate::port::scg::Pin for $pin {
//             type Module = crate::port::consts::$module;
//             type Signal = crate::port::scg::$signal;
//         }
//     };
// }

// // SCG XTAL/EXTAL pins are always P1_30/P1_31
// scg!(pin: PortPin<1, 30>, module: U0, signal: XTAL);
// scg!(pin: PortPin<1, 31>, module: U0, signal: EXTAL);

// // pub(crate) use scg;

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
            type Module = crate::consts::$module;
            type Signal = crate::port::scg::$signal;
        }
    };
}
pub(crate) use scg;
