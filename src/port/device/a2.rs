use crate::{
    port::PortPin,
    syscon::{PeripheralCC, PeripheralRST},
};

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
}
impl crate::private::Sealed for Port0 {}
unsafe impl Send for Port0 {}
unsafe impl Sync for Port0 {}
impl Port0 {
    #[inline(always)]
    pub fn new(mut port: crate::pac::port::PORT0) -> Self {
        port.reset(true);
        port.clock(true);
        unsafe {
            Self {
                _port: port,
                p0: PortPin::new(),
                p1: PortPin::new(),
                p2: PortPin::new(),
                p3: PortPin::new(),
                p4: PortPin::new(),
                p5: PortPin::new(),
                p6: PortPin::new(),
                p7: PortPin::new(),
                p12: PortPin::new(),
                p13: PortPin::new(),
                p14: PortPin::new(),
                p15: PortPin::new(),
                p16: PortPin::new(),
                p17: PortPin::new(),
                p18: PortPin::new(),
                p19: PortPin::new(),
                p20: PortPin::new(),
                p21: PortPin::new(),
                p22: PortPin::new(),
                p23: PortPin::new(),
                p24: PortPin::new(),
                p25: PortPin::new(),
                p26: PortPin::new(),
                p27: PortPin::new(),
            }
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
    pub p29: PortPin<1, 29>,
    pub p30: PortPin<1, 30>,
    pub p31: PortPin<1, 31>,
}
impl crate::private::Sealed for Port1 {}
unsafe impl Send for Port1 {}
unsafe impl Sync for Port1 {}
impl Port1 {
    #[inline(always)]
    pub fn new(mut port: crate::pac::port::PORT1) -> Self {
        port.reset(true);
        port.clock(true);
        unsafe {
            Self {
                _port: port,
                p0: PortPin::new(),
                p1: PortPin::new(),
                p2: PortPin::new(),
                p3: PortPin::new(),
                p4: PortPin::new(),
                p5: PortPin::new(),
                p6: PortPin::new(),
                p7: PortPin::new(),
                p8: PortPin::new(),
                p9: PortPin::new(),
                p10: PortPin::new(),
                p11: PortPin::new(),
                p12: PortPin::new(),
                p13: PortPin::new(),
                p14: PortPin::new(),
                p15: PortPin::new(),
                p16: PortPin::new(),
                p17: PortPin::new(),
                p18: PortPin::new(),
                p19: PortPin::new(),
                p29: PortPin::new(),
                p30: PortPin::new(),
                p31: PortPin::new(),
            }
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
    pub p12: PortPin<2, 12>,
    pub p13: PortPin<2, 13>,
    pub p14: PortPin<2, 14>,
    pub p15: PortPin<2, 15>,
    pub p16: PortPin<2, 16>,
    pub p17: PortPin<2, 17>,
    pub p18: PortPin<2, 18>,
    pub p19: PortPin<2, 19>,
    pub p20: PortPin<2, 20>,
    pub p21: PortPin<2, 21>,
    pub p22: PortPin<2, 22>,
    pub p23: PortPin<2, 23>,
    pub p24: PortPin<2, 24>,
    pub p25: PortPin<2, 25>,
    pub p26: PortPin<2, 26>,
}
impl crate::private::Sealed for Port2 {}
unsafe impl Send for Port2 {}
unsafe impl Sync for Port2 {}
impl Port2 {
    #[inline(always)]
    pub fn new(mut port: crate::pac::port::PORT2) -> Self {
        port.reset(true);
        port.clock(true);
        unsafe {
            Self {
                _port: port,
                p0: PortPin::new(),
                p1: PortPin::new(),
                p2: PortPin::new(),
                p3: PortPin::new(),
                p4: PortPin::new(),
                p5: PortPin::new(),
                p6: PortPin::new(),
                p7: PortPin::new(),
                p8: PortPin::new(),
                p9: PortPin::new(),
                p10: PortPin::new(),
                p11: PortPin::new(),
                p12: PortPin::new(),
                p13: PortPin::new(),
                p14: PortPin::new(),
                p15: PortPin::new(),
                p16: PortPin::new(),
                p17: PortPin::new(),
                p18: PortPin::new(),
                p19: PortPin::new(),
                p20: PortPin::new(),
                p21: PortPin::new(),
                p22: PortPin::new(),
                p23: PortPin::new(),
                p24: PortPin::new(),
                p25: PortPin::new(),
                p26: PortPin::new(),
            }
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
    pub p24: PortPin<3, 24>,
    pub p25: PortPin<3, 25>,
    pub p26: PortPin<3, 26>,
    pub p27: PortPin<3, 27>,
    pub p28: PortPin<3, 28>,
    pub p29: PortPin<3, 29>,
    pub p30: PortPin<3, 30>,
    pub p31: PortPin<3, 31>,
}
impl crate::private::Sealed for Port3 {}
unsafe impl Send for Port3 {}
unsafe impl Sync for Port3 {}
impl Port3 {
    #[inline(always)]
    pub fn new(mut port: crate::pac::port::PORT3) -> Self {
        port.reset(true);
        port.clock(true);
        unsafe {
            Self {
                _port: port,
                p0: PortPin::new(),
                p1: PortPin::new(),
                p2: PortPin::new(),
                p3: PortPin::new(),
                p4: PortPin::new(),
                p5: PortPin::new(),
                p6: PortPin::new(),
                p7: PortPin::new(),
                p8: PortPin::new(),
                p9: PortPin::new(),
                p10: PortPin::new(),
                p11: PortPin::new(),
                p12: PortPin::new(),
                p13: PortPin::new(),
                p14: PortPin::new(),
                p15: PortPin::new(),
                p16: PortPin::new(),
                p17: PortPin::new(),
                p18: PortPin::new(),
                p19: PortPin::new(),
                p20: PortPin::new(),
                p21: PortPin::new(),
                p22: PortPin::new(),
                p23: PortPin::new(),
                p24: PortPin::new(),
                p25: PortPin::new(),
                p26: PortPin::new(),
                p27: PortPin::new(),
                p28: PortPin::new(),
                p29: PortPin::new(),
                p30: PortPin::new(),
                p31: PortPin::new(),
            }
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
}
impl crate::private::Sealed for Port4 {}
unsafe impl Send for Port4 {}
unsafe impl Sync for Port4 {}
impl Port4 {
    #[inline(always)]
    pub fn new(mut port: crate::pac::port::PORT4) -> Self {
        port.reset(true);
        port.clock(true);
        unsafe {
            Self {
                _port: port,
                p0: PortPin::new(),
                p1: PortPin::new(),
                p2: PortPin::new(),
                p3: PortPin::new(),
                p4: PortPin::new(),
                p5: PortPin::new(),
                p6: PortPin::new(),
                p7: PortPin::new(),
            }
        }
    }
}
