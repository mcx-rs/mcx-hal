use crate::port::PortPin;
use crate::private;
use crate::syscon::{PeripheralCC, PeripheralRST};

use crate::port::{scg, lpuart};
pub struct Port0 {
    _port: crate::pac::port::PORT0,
    pub p0: PortPin<0, 0>,
    pub p1: PortPin<0, 1>,
    pub p2: PortPin<0, 2>,
    pub p3: PortPin<0, 3>,
    pub p4: PortPin<0, 4>,
    pub p5: PortPin<0, 5>,
    pub p6: PortPin<0, 6>,
    pub p7: PortPin<0, 7>,
    pub p8: PortPin<0, 8>,
    pub p9: PortPin<0, 9>,
    pub p10: PortPin<0, 10>,
    pub p11: PortPin<0, 11>,
    pub p12: PortPin<0, 12>,
    pub p13: PortPin<0, 13>,
    pub p14: PortPin<0, 14>,
    pub p15: PortPin<0, 15>,
    pub p16: PortPin<0, 16>,
    pub p17: PortPin<0, 17>,
    pub p18: PortPin<0, 18>,
    pub p19: PortPin<0, 19>,
    pub p20: PortPin<0, 20>,
    pub p21: PortPin<0, 21>,
    pub p22: PortPin<0, 22>,
    pub p23: PortPin<0, 23>,
    pub p24: PortPin<0, 24>,
    pub p25: PortPin<0, 25>,
    pub p26: PortPin<0, 26>,
    pub p27: PortPin<0, 27>,
    pub p28: PortPin<0, 28>,
    pub p29: PortPin<0, 29>,
    pub p30: PortPin<0, 30>,
    pub p31: PortPin<0, 31>,
}
impl private::Sealed for Port0 {}
unsafe impl Send for Port0 {}
unsafe impl Sync for Port0 {}
impl Port0 {
    pub fn new(mut port: crate::pac::port::PORT0) -> Self {
        port.reset(true);
        port.clock(true);
        Self {
            _port: port,
            p0: unsafe { PortPin::<0, 0>::new() },
            p1: unsafe { PortPin::<0, 1>::new() },
            p2: unsafe { PortPin::<0, 2>::new() },
            p3: unsafe { PortPin::<0, 3>::new() },
            p4: unsafe { PortPin::<0, 4>::new() },
            p5: unsafe { PortPin::<0, 5>::new() },
            p6: unsafe { PortPin::<0, 6>::new() },
            p7: unsafe { PortPin::<0, 7>::new() },
            p8: unsafe { PortPin::<0, 8>::new() },
            p9: unsafe { PortPin::<0, 9>::new() },
            p10: unsafe { PortPin::<0, 10>::new() },
            p11: unsafe { PortPin::<0, 11>::new() },
            p12: unsafe { PortPin::<0, 12>::new() },
            p13: unsafe { PortPin::<0, 13>::new() },
            p14: unsafe { PortPin::<0, 14>::new() },
            p15: unsafe { PortPin::<0, 15>::new() },
            p16: unsafe { PortPin::<0, 16>::new() },
            p17: unsafe { PortPin::<0, 17>::new() },
            p18: unsafe { PortPin::<0, 18>::new() },
            p19: unsafe { PortPin::<0, 19>::new() },
            p20: unsafe { PortPin::<0, 20>::new() },
            p21: unsafe { PortPin::<0, 21>::new() },
            p22: unsafe { PortPin::<0, 22>::new() },
            p23: unsafe { PortPin::<0, 23>::new() },
            p24: unsafe { PortPin::<0, 24>::new() },
            p25: unsafe { PortPin::<0, 25>::new() },
            p26: unsafe { PortPin::<0, 26>::new() },
            p27: unsafe { PortPin::<0, 27>::new() },
            p28: unsafe { PortPin::<0, 28>::new() },
            p29: unsafe { PortPin::<0, 29>::new() },
            p30: unsafe { PortPin::<0, 30>::new() },
            p31: unsafe { PortPin::<0, 31>::new() },
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
    pub p14: PortPin<1, 14>,
    pub p15: PortPin<1, 15>,
    pub p16: PortPin<1, 16>,
    pub p17: PortPin<1, 17>,
    pub p18: PortPin<1, 18>,
    pub p19: PortPin<1, 19>,
    pub p20: PortPin<1, 20>,
    pub p21: PortPin<1, 21>,
    pub p22: PortPin<1, 22>,
    pub p23: PortPin<1, 23>,
    pub p30: PortPin<1, 30>,
    pub p31: PortPin<1, 31>,
}
impl private::Sealed for Port1 {}
unsafe impl Send for Port1 {}
unsafe impl Sync for Port1 {}
impl Port1 {
    pub fn new(mut port: crate::pac::port::PORT1) -> Self {
        port.reset(true);
        port.clock(true);
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
            p14: unsafe { PortPin::<1, 14>::new() },
            p15: unsafe { PortPin::<1, 15>::new() },
            p16: unsafe { PortPin::<1, 16>::new() },
            p17: unsafe { PortPin::<1, 17>::new() },
            p18: unsafe { PortPin::<1, 18>::new() },
            p19: unsafe { PortPin::<1, 19>::new() },
            p20: unsafe { PortPin::<1, 20>::new() },
            p21: unsafe { PortPin::<1, 21>::new() },
            p22: unsafe { PortPin::<1, 22>::new() },
            p23: unsafe { PortPin::<1, 23>::new() },
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
    pub p8: PortPin<2, 8>,
    pub p9: PortPin<2, 9>,
    pub p10: PortPin<2, 10>,
    pub p11: PortPin<2, 11>,
}
impl private::Sealed for Port2 {}
unsafe impl Send for Port2 {}
unsafe impl Sync for Port2 {}
impl Port2 {
    pub fn new(mut port: crate::pac::port::PORT2) -> Self {
        port.reset(true);
        port.clock(true);
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
            p8: unsafe { PortPin::<2, 8>::new() },
            p9: unsafe { PortPin::<2, 9>::new() },
            p10: unsafe { PortPin::<2, 10>::new() },
            p11: unsafe { PortPin::<2, 11>::new() },
        }
    }
}
pub struct Port3 {
    _port: crate::pac::port::PORT3,
    pub p0: PortPin<3, 0>,
    pub p1: PortPin<3, 1>,
    pub p2: PortPin<3, 2>,
    pub p3: PortPin<3, 3>,
    pub p4: PortPin<3, 4>,
    pub p5: PortPin<3, 5>,
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
    pub p16: PortPin<3, 16>,
    pub p17: PortPin<3, 17>,
    pub p18: PortPin<3, 18>,
    pub p19: PortPin<3, 19>,
    pub p20: PortPin<3, 20>,
    pub p21: PortPin<3, 21>,
    pub p22: PortPin<3, 22>,
    pub p23: PortPin<3, 23>,
}
impl private::Sealed for Port3 {}
unsafe impl Send for Port3 {}
unsafe impl Sync for Port3 {}
impl Port3 {
    pub fn new(mut port: crate::pac::port::PORT3) -> Self {
        port.reset(true);
        port.clock(true);
        Self {
            _port: port,
            p0: unsafe { PortPin::<3, 0>::new() },
            p1: unsafe { PortPin::<3, 1>::new() },
            p2: unsafe { PortPin::<3, 2>::new() },
            p3: unsafe { PortPin::<3, 3>::new() },
            p4: unsafe { PortPin::<3, 4>::new() },
            p5: unsafe { PortPin::<3, 5>::new() },
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
            p16: unsafe { PortPin::<3, 16>::new() },
            p17: unsafe { PortPin::<3, 17>::new() },
            p18: unsafe { PortPin::<3, 18>::new() },
            p19: unsafe { PortPin::<3, 19>::new() },
            p20: unsafe { PortPin::<3, 20>::new() },
            p21: unsafe { PortPin::<3, 21>::new() },
            p22: unsafe { PortPin::<3, 22>::new() },
            p23: unsafe { PortPin::<3, 23>::new() },
        }
    }
}
pub struct Port4 {
    _port: crate::pac::port::PORT4,
    pub p0: PortPin<4, 0>,
    pub p1: PortPin<4, 1>,
    pub p2: PortPin<4, 2>,
    pub p3: PortPin<4, 3>,
    pub p4: PortPin<4, 4>,
    pub p5: PortPin<4, 5>,
    pub p6: PortPin<4, 6>,
    pub p7: PortPin<4, 7>,
    pub p12: PortPin<4, 12>,
    pub p13: PortPin<4, 13>,
    pub p14: PortPin<4, 14>,
    pub p15: PortPin<4, 15>,
    pub p16: PortPin<4, 16>,
    pub p17: PortPin<4, 17>,
    pub p18: PortPin<4, 18>,
    pub p19: PortPin<4, 19>,
    pub p20: PortPin<4, 20>,
    pub p21: PortPin<4, 21>,
    pub p22: PortPin<4, 22>,
    pub p23: PortPin<4, 23>,
}
impl private::Sealed for Port4 {}
unsafe impl Send for Port4 {}
unsafe impl Sync for Port4 {}
impl Port4 {
    pub fn new(mut port: crate::pac::port::PORT4) -> Self {
        port.reset(true);
        port.clock(true);
        Self {
            _port: port,
            p0: unsafe { PortPin::<4, 0>::new() },
            p1: unsafe { PortPin::<4, 1>::new() },
            p2: unsafe { PortPin::<4, 2>::new() },
            p3: unsafe { PortPin::<4, 3>::new() },
            p4: unsafe { PortPin::<4, 4>::new() },
            p5: unsafe { PortPin::<4, 5>::new() },
            p6: unsafe { PortPin::<4, 6>::new() },
            p7: unsafe { PortPin::<4, 7>::new() },
            p12: unsafe { PortPin::<4, 12>::new() },
            p13: unsafe { PortPin::<4, 13>::new() },
            p14: unsafe { PortPin::<4, 14>::new() },
            p15: unsafe { PortPin::<4, 15>::new() },
            p16: unsafe { PortPin::<4, 16>::new() },
            p17: unsafe { PortPin::<4, 17>::new() },
            p18: unsafe { PortPin::<4, 18>::new() },
            p19: unsafe { PortPin::<4, 19>::new() },
            p20: unsafe { PortPin::<4, 20>::new() },
            p21: unsafe { PortPin::<4, 21>::new() },
            p22: unsafe { PortPin::<4, 22>::new() },
            p23: unsafe { PortPin::<4, 23>::new() },
        }
    }
}
pub struct Port5 {
    _port: crate::pac::port::PORT5,
    pub p0: PortPin<5, 0>,
    pub p1: PortPin<5, 1>,
    pub p2: PortPin<5, 2>,
    pub p3: PortPin<5, 3>,
    pub p4: PortPin<5, 4>,
    pub p5: PortPin<5, 5>,
    pub p6: PortPin<5, 6>,
    pub p7: PortPin<5, 7>,
    pub p8: PortPin<5, 8>,
    pub p9: PortPin<5, 9>,
}
impl private::Sealed for Port5 {}
unsafe impl Send for Port5 {}
unsafe impl Sync for Port5 {}
impl Port5 {
    pub fn new(mut port: crate::pac::port::PORT5) -> Self {
        port.reset(true);
        port.clock(true);
        Self {
            _port: port,
            p0: unsafe { PortPin::<5, 0>::new() },
            p1: unsafe { PortPin::<5, 1>::new() },
            p2: unsafe { PortPin::<5, 2>::new() },
            p3: unsafe { PortPin::<5, 3>::new() },
            p4: unsafe { PortPin::<5, 4>::new() },
            p5: unsafe { PortPin::<5, 5>::new() },
            p6: unsafe { PortPin::<5, 6>::new() },
            p7: unsafe { PortPin::<5, 7>::new() },
            p8: unsafe { PortPin::<5, 8>::new() },
            p9: unsafe { PortPin::<5, 9>::new() },
        }
    }
}
scg!(pin: PortPin<1, 30>, module: U0, signal: XTAL48M)
scg!(pin: PortPin<1, 31>, module: U0, signal: EXTAL48M)
