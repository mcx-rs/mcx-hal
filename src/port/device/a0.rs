use crate::port::PortPin;
use crate::private;
use crate::syscon::{PeripheralCC, PeripheralRST};

use crate::port::{lpuart, scg};
pub struct Port0 {
    _port: crate::pac::port::PORT0,
    pub p0: PortPin<0, 0>,
    pub p1: PortPin<0, 1>,
    pub p2: PortPin<0, 2>,
    pub p3: PortPin<0, 3>,
    pub p6: PortPin<0, 6>,
    pub p16: PortPin<0, 16>,
    pub p17: PortPin<0, 17>,
}
impl private::Sealed for Port0 {}
unsafe impl Send for Port0 {}
unsafe impl Sync for Port0 {}
impl Port0 {
    pub fn new(mut port: crate::pac::port::PORT0) -> Self {
        port.reset();
        port.enable_clock(true);
        Self {
            _port: port,
            p0: unsafe { PortPin::<0, 0>::new() },
            p1: unsafe { PortPin::<0, 1>::new() },
            p2: unsafe { PortPin::<0, 2>::new() },
            p3: unsafe { PortPin::<0, 3>::new() },
            p6: unsafe { PortPin::<0, 6>::new() },
            p16: unsafe { PortPin::<0, 16>::new() },
            p17: unsafe { PortPin::<0, 17>::new() },
        }
    }
}
pub struct Port1 {
    _port: crate::pac::port::PORT1,
    pub p0: PortPin<1, 0>,
    pub p1: PortPin<1, 1>,
    pub p2: PortPin<1, 2>,
    pub p3: PortPin<1, 3>,
    pub p4: PortPin<1, 4>,
    pub p5: PortPin<1, 5>,
    pub p6: PortPin<1, 6>,
    pub p7: PortPin<1, 7>,
    pub p8: PortPin<1, 8>,
    pub p9: PortPin<1, 9>,
    pub p10: PortPin<1, 10>,
    pub p11: PortPin<1, 11>,
    pub p12: PortPin<1, 12>,
    pub p13: PortPin<1, 13>,
    pub p29: PortPin<1, 29>,
    pub p30: PortPin<1, 30>,
    pub p31: PortPin<1, 31>,
}
impl private::Sealed for Port1 {}
unsafe impl Send for Port1 {}
unsafe impl Sync for Port1 {}
impl Port1 {
    pub fn new(mut port: crate::pac::port::PORT1) -> Self {
        port.reset();
        port.enable_clock(true);
        Self {
            _port: port,
            p0: unsafe { PortPin::<1, 0>::new() },
            p1: unsafe { PortPin::<1, 1>::new() },
            p2: unsafe { PortPin::<1, 2>::new() },
            p3: unsafe { PortPin::<1, 3>::new() },
            p4: unsafe { PortPin::<1, 4>::new() },
            p5: unsafe { PortPin::<1, 5>::new() },
            p6: unsafe { PortPin::<1, 6>::new() },
            p7: unsafe { PortPin::<1, 7>::new() },
            p8: unsafe { PortPin::<1, 8>::new() },
            p9: unsafe { PortPin::<1, 9>::new() },
            p10: unsafe { PortPin::<1, 10>::new() },
            p11: unsafe { PortPin::<1, 11>::new() },
            p12: unsafe { PortPin::<1, 12>::new() },
            p13: unsafe { PortPin::<1, 13>::new() },
            p29: unsafe { PortPin::<1, 29>::new() },
            p30: unsafe { PortPin::<1, 30>::new() },
            p31: unsafe { PortPin::<1, 31>::new() },
        }
    }
}
pub struct Port2 {
    _port: crate::pac::port::PORT2,
    pub p0: PortPin<2, 0>,
    pub p1: PortPin<2, 1>,
    pub p2: PortPin<2, 2>,
    pub p3: PortPin<2, 3>,
    pub p4: PortPin<2, 4>,
    pub p5: PortPin<2, 5>,
    pub p6: PortPin<2, 6>,
    pub p7: PortPin<2, 7>,
    pub p12: PortPin<2, 12>,
    pub p13: PortPin<2, 13>,
    pub p16: PortPin<2, 16>,
    pub p17: PortPin<2, 17>,
    pub p20: PortPin<2, 20>,
    pub p21: PortPin<2, 21>,
}
impl private::Sealed for Port2 {}
unsafe impl Send for Port2 {}
unsafe impl Sync for Port2 {}
impl Port2 {
    pub fn new(mut port: crate::pac::port::PORT2) -> Self {
        port.reset();
        port.enable_clock(true);
        Self {
            _port: port,
            p0: unsafe { PortPin::<2, 0>::new() },
            p1: unsafe { PortPin::<2, 1>::new() },
            p2: unsafe { PortPin::<2, 2>::new() },
            p3: unsafe { PortPin::<2, 3>::new() },
            p4: unsafe { PortPin::<2, 4>::new() },
            p5: unsafe { PortPin::<2, 5>::new() },
            p6: unsafe { PortPin::<2, 6>::new() },
            p7: unsafe { PortPin::<2, 7>::new() },
            p12: unsafe { PortPin::<2, 12>::new() },
            p13: unsafe { PortPin::<2, 13>::new() },
            p16: unsafe { PortPin::<2, 16>::new() },
            p17: unsafe { PortPin::<2, 17>::new() },
            p20: unsafe { PortPin::<2, 20>::new() },
            p21: unsafe { PortPin::<2, 21>::new() },
        }
    }
}
pub struct Port3 {
    _port: crate::pac::port::PORT3,
    pub p0: PortPin<3, 0>,
    pub p1: PortPin<3, 1>,
    pub p6: PortPin<3, 6>,
    pub p7: PortPin<3, 7>,
    pub p8: PortPin<3, 8>,
    pub p9: PortPin<3, 9>,
    pub p10: PortPin<3, 10>,
    pub p11: PortPin<3, 11>,
    pub p12: PortPin<3, 12>,
    pub p13: PortPin<3, 13>,
    pub p14: PortPin<3, 14>,
    pub p15: PortPin<3, 15>,
    pub p27: PortPin<3, 27>,
    pub p28: PortPin<3, 28>,
    pub p29: PortPin<3, 29>,
    pub p30: PortPin<3, 30>,
    pub p31: PortPin<3, 31>,
}
impl private::Sealed for Port3 {}
unsafe impl Send for Port3 {}
unsafe impl Sync for Port3 {}
impl Port3 {
    pub fn new(mut port: crate::pac::port::PORT3) -> Self {
        port.reset();
        port.enable_clock(true);
        Self {
            _port: port,
            p0: unsafe { PortPin::<3, 0>::new() },
            p1: unsafe { PortPin::<3, 1>::new() },
            p6: unsafe { PortPin::<3, 6>::new() },
            p7: unsafe { PortPin::<3, 7>::new() },
            p8: unsafe { PortPin::<3, 8>::new() },
            p9: unsafe { PortPin::<3, 9>::new() },
            p10: unsafe { PortPin::<3, 10>::new() },
            p11: unsafe { PortPin::<3, 11>::new() },
            p12: unsafe { PortPin::<3, 12>::new() },
            p13: unsafe { PortPin::<3, 13>::new() },
            p14: unsafe { PortPin::<3, 14>::new() },
            p15: unsafe { PortPin::<3, 15>::new() },
            p27: unsafe { PortPin::<3, 27>::new() },
            p28: unsafe { PortPin::<3, 28>::new() },
            p29: unsafe { PortPin::<3, 29>::new() },
            p30: unsafe { PortPin::<3, 30>::new() },
            p31: unsafe { PortPin::<3, 31>::new() },
        }
    }
}
lpuart!(pin: PortPin<0, 2>, module: U0, signal: RXD, mux: 2);
lpuart!(pin: PortPin<0, 3>, module: U0, signal: TXD, mux: 2);
lpuart!(pin: PortPin<1, 12>, module: U2, signal: RXD, mux: 3);
lpuart!(pin: PortPin<1, 13>, module: U2, signal: TXD, mux: 3);
scg!(pin: PortPin<1, 30>, module: U0, signal: XTAL48M);
scg!(pin: PortPin<1, 31>, module: U0, signal: EXTAL48M);
lpuart!(pin: PortPin<1, 4>, module: U2, signal: RXD, mux: 3);
lpuart!(pin: PortPin<1, 5>, module: U2, signal: TXD, mux: 3);
lpuart!(pin: PortPin<1, 8>, module: U1, signal: RXD, mux: 2);
lpuart!(pin: PortPin<1, 9>, module: U1, signal: TXD, mux: 2);
lpuart!(pin: PortPin<2, 0>, module: U0, signal: RXD, mux: 2);
lpuart!(pin: PortPin<2, 1>, module: U0, signal: TXD, mux: 2);
lpuart!(pin: PortPin<2, 12>, module: U1, signal: RXD, mux: 3);
lpuart!(pin: PortPin<2, 13>, module: U1, signal: TXD, mux: 3);
lpuart!(pin: PortPin<2, 2>, module: U2, signal: TXD, mux: 3);
lpuart!(pin: PortPin<2, 3>, module: U2, signal: RXD, mux: 3);
lpuart!(pin: PortPin<3, 14>, module: U2, signal: RXD, mux: 2);
lpuart!(pin: PortPin<3, 15>, module: U2, signal: TXD, mux: 2);
lpuart!(pin: PortPin<3, 8>, module: U1, signal: RXD, mux: 3);
lpuart!(pin: PortPin<3, 9>, module: U1, signal: TXD, mux: 3);
