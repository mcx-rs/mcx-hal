//! MRCC System and Peripherals clock set and control

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

/// Peripheral in MRCC.
///
/// Use `N` to choose related register, and `BIT` to choose related bit.
pub trait PeripheralMRCC: crate::private::Sealed {
    const N: usize;
    const BIT: usize;
}

/// Peripheral RST Control feature in MRCC.
pub trait PeripheralRST: PeripheralMRCC {
    fn reset(&mut self, release: bool);
}

/// AHB Clock Control feature in MRCC.
pub trait PeripheralCC: PeripheralMRCC {
    fn clock(&mut self, enable: bool);
}

/// Automatic Clock Gating feature in MRCC.
pub trait PeripheralACC: PeripheralMRCC {
    fn auto_clock_gating(&mut self, enable: bool);
}

macro_rules! mrcc_periph_en_define {
    ( $( ( $(virt: $virt:ident)? $(periph: $periph:ty)?, $n:expr, $bit:expr $(, hRST: $hRST:expr)? $(, hCC: $hCC:expr)? $(, hACC: $hACC:expr)? ) )* ) => {
        // mrcc_periph_en_define!(@common_define);

        mrcc_periph_en_define!(@define_virtual_peripherals $( $($virt)? )+);
        $($(
            impl crate::private::Sealed for $periph {}
        )?)*
        pub use virtual_peripherals::*;
        $(
            mrcc_periph_en_define!($(virt: $virt)? $(periph: $periph)?, $n, $bit $(, hRST: $hRST)? $(, hCC: $hCC)? $(, hACC: $hACC)?);
        )*

    };

    ( $(virt: $virt:ident)? $(periph: $periph:ty)?, $n:expr, $bit:expr $(, hRST: $hRST:expr)? $(, hCC: $hCC:expr)? $(, hACC: $hACC:expr)? ) => {
        // $(
        //     pub struct $virt;
        //     impl crate::private::Sealed for $virt {};
        // )?

        mrcc_periph_en_define!(@impl_mrcc mrcc_periph_en_define!(@name $(virt: $virt)? $(periph: $periph)?), $n, $bit);
        mrcc_periph_en_define!(@impl_rst mrcc_periph_en_define!(@name $(virt: $virt)? $(periph: $periph)?) $(, hRST: $hRST)?);
        mrcc_periph_en_define!(@impl_cc mrcc_periph_en_define!(@name $(virt: $virt)? $(periph: $periph)?) $(, hCC: $hCC)?);
        mrcc_periph_en_define!(@impl_acc mrcc_periph_en_define!(@name $(virt: $virt)? $(periph: $periph)?) $(, hACC: $hACC)?);
    };

    (@define_virtual_peripherals $($virt:ident)*) => {
        pub mod virtual_peripherals {
            $(
                pub struct $virt;
                impl crate::private::Sealed for $virt {}
            )*
        }
    };

    (@common_define) => {
        pub trait PeripheralMRCC: crate::private::Sealed {
            const N: usize;
            const BIT: usize;
        }
        pub trait PeripheralRST: PeripheralMRCC {
            fn reset(&mut self, release: bool);
        }
        pub trait PeripheralCC: PeripheralMRCC {
            fn clock(&mut self, enable: bool);
        }
        pub trait PeripheralACC: PeripheralMRCC {
            fn auto_clock_gating(&mut self, enable: bool);
        }
    };

    (@impl_mrcc $name:ty, $n:expr, $bit:expr) => {
        impl PeripheralMRCC for $name {
            const N: usize = $n;
            const BIT: usize = $bit;
        }
    };

    (@name $(virt: $virt:ident)? $(periph: $periph:ty)?) => {
        $($virt)?
        $($periph)?
    };
    (@impl_rst $name:ty, hRST: $hRST:expr) => {
        impl PeripheralRST for $name {
            #[inline(always)]
            fn reset(&mut self, release: bool) {
                let reg: crate::pac::common::Reg<u32, crate::pac::common::W> = unsafe {
                    let ptr = mrcc::ADDRESSES[0] as *mut u8;
                    let offset = if release { 0x04usize } else { 0x08usize };
                    crate::pac::common::Reg::from_ptr(ptr.add(0x00usize + Self::N * 0x10usize + offset) as _)
                };
                reg.write(|r| *r = (1 << Self::BIT));
            }
        }
    };
    (@impl_rst $name:ty) => {};
    (@impl_cc $name:ty, hCC: $hCC:expr) => {
        impl PeripheralCC for $name {
            #[inline(always)]
            fn clock(&mut self, enable: bool) {
                let reg: crate::pac::common::Reg<u32, crate::pac::common::W> = unsafe {
                    let ptr = mrcc::ADDRESSES[0] as *mut u8;
                    let offset = if enable { 0x04usize } else { 0x08usize };
                    crate::pac::common::Reg::from_ptr(ptr.add(0x40usize + Self::N * 0x10usize + offset) as _)
                };
                reg.write(|r| *r = (1 << Self::BIT));
            }
        }
    };
    (@impl_cc $name:ty) => {};
    (@impl_acc $name:ty, hACC: $hACC:expr) => {
        impl PeripheralACC for $name {
            #[inline(always)]
            fn auto_clock_gating(&mut self, enable: bool) {
                let reg: crate::pac::common::Reg<u32, crate::pac::common::RW> = unsafe {
                    let ptr = mrcc::ADDRESSES[0] as *mut u8;
                    crate::pac::common::Reg::from_ptr(ptr.add(0x80usize + Self::N * 0x04usize) as _)
                };
                reg.modify(|r| {
                    if enable {
                         *r ^= (1 << Self::BIT);
                    } else {
                        *r &= !(1 << Self::BIT);
                    }
                });
            }
        }
    };
    (@impl_acc $name:ty) => {};
}
pub(crate) use mrcc_periph_en_define;
