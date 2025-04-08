//! System Controller

mod syscon_divider;
pub use syscon_divider::*;

#[cfg(feature = "mcxa")]
mod mrcc_divider;
#[cfg(feature = "mcxa")]
pub(crate) use mrcc_divider::*;

use crate::private;

pub trait PeripheralRST: private::Sealed {
    fn assert_reset(&mut self, release: bool);
    fn reset(&mut self) {
        self.assert_reset(true);
        self.assert_reset(true);
    }
}
pub trait PeripheralCC: private::Sealed {
    fn enable_clock(&mut self, enable: bool);
}
pub trait PeripheralEn: private::Sealed {
    fn enable(enable: bool);
}

#[cfg(feature = "mcxa")]
macro_rules! periph_en_define {
    ( $( ($(virt: $virt:ident)? $(periph: $periph:ty)?, $n:expr, $bit:expr $(,hRST: $hRST:expr)? $(,hCC: $hCC:expr)? $(,hACC: $hACC:expr)?) )+ ) => {
        periph_en_define!(@virtual_peripherals $($($virt)?)+);
        $($(
            impl crate::private::Sealed for $periph {}
        )?)+

        pub use virtual_peripherals::*;
        $(
            periph_en_define!(@impl_rst periph_en_define!(@name $(virt: $virt)? $(periph: $periph)?), $n, $bit $(,hRST: $hRST)?);
            periph_en_define!(@impl_cc  periph_en_define!(@name $(virt: $virt)? $(periph: $periph)?), $n, $bit $(,hCC: $hCC)?);
        )+
    };
    (@name $(virt: $virt:ident)? $(periph: $periph:ty)?) => {
        $($virt)?
        $($periph)?
    };
    (@virtual_peripherals $($virt:ident)*) => {
        pub mod virtual_peripherals {
            $(
                pub struct $virt;
                impl crate::private::Sealed for $virt {}
            )*
        }
    };
    (@impl_rst $name:ty, $n:expr, $bit:expr) => {};
    (@impl_rst $name:ty, $n:expr, $bit:expr, hRST: $hRST:expr) => {
        impl crate::syscon::PeripheralRST for $name {
            #[inline(always)]
            fn assert_reset(&mut self, release: bool) {
                let reg = unsafe {
                    let ptr = crate::pac::mrcc::ADDRESSES[0] as *mut u8;
                    let offset = if release { 0x04usize } else { 0x08usize };
                    crate::pac::common::Reg::<u32, crate::pac::common::W>::from_ptr(ptr.add(0x00usize + $n * 0x10usize + offset) as _)
                };
                reg.write(|r| *r = (1 << $bit));
            }
        }
    };
    (@impl_cc $name:ty, $n:expr, $bit:expr) => {};
    (@impl_cc $name:ty, $n:expr, $bit:expr, hCC: $hCC:expr) => {
        impl crate::syscon::PeripheralCC for $name {
            #[inline(always)]
            fn enable_clock(&mut self, enable: bool) {
                let reg = unsafe {
                    let ptr = crate::pac::mrcc::ADDRESSES[0] as *mut u8;
                    let offset = if enable { 0x04usize } else { 0x08usize };
                    crate::pac::common::Reg::<u32, crate::pac::common::W>::from_ptr(ptr.add(0x40usize + $n * 0x10usize + offset) as _)
                };
                reg.write(|r| *r = (1 << $bit));
            }
        }
    };
    // (@impl_acc $name:ty, $n:expr, $bit:expr, hACC: $hACC:expr) => {
    //     impl crate::syscon::PeripheralRST for $name {
    //         #[inline(always)]
    //         fn reset(&mut self, release: bool) {
    //             let reg = unsafe {
    //                 let ptr = crate::pac::mrcc::ADDRESSES[0] as *mut u8;
    //                 let offset = if release { 0x04usize } else { 0x08usize };
    //                 crate::pac::common::Reg::<u32, crate::pac::common::W>::from_ptr(ptr.add(0x00usize + $n * 0x10usize + offset) as _)
    //             };
    //             reg.write(|r| *r = (1 << $bit));
    //         }
    //     }
    // };
}

#[cfg(feature = "mcxn")]
macro_rules! periph_en_define {
    ( $( ($(virt: $virt:ident)? $(periph: $periph:ty)?, $n:expr, $bit:expr $(,hRST: $hRST:expr)? $(,hCC: $hCC:expr)? $(,hACC: $hACC:expr)?) )+ ) => {};
}

pub(crate) use periph_en_define;

cfg_if::cfg_if! {
    if #[cfg(feature = "mcxa")] {
        mod mrcc;
    }
}

// mod device {
//     cfg_if::cfg_if! {
//         if #[cfg(feature = "mcxa0")] {
//             mod a0 as _d;
//         } else if #[cfg(feature = "mcxa1")] {
//             mod a1 as _d;
//         } else if #[cfg(feature = "mcxa2")] {
//             mod a2 as _d;
//         }
//     }
// }

#[cfg_attr(feature = "mcxa0", path = "device/a0.rs")]
#[cfg_attr(feature = "mcxa1", path = "device/a1.rs")]
#[cfg_attr(feature = "mcxa2", path = "device/a2.rs")]
mod device;
pub use device::*;
