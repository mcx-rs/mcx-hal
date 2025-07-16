macro_rules! periph_mrcc {
    ( $( ( $(virt: $virt:ident)? $(periph: $periph:ty)?, $n:expr, $bit:expr $(, hRST: $hRST:expr)? $(, hCC: $hCC:expr)? $(, hACC: $hACC:expr)? $(, features: $features:meta)? ) )+ ) => {
        $( periph_mrcc!($(virt: $virt)? $(periph: $periph)?, $n, $bit $(, hRST: $hRST)? $(, hCC: $hCC)? $(, hACC: $hACC)? $(, features: $features)?); )+
    };

    (virt: $virt:ident, $n:expr, $bit:expr $(, hRST: $hRST:expr)? $(, hCC: $hCC:expr)? $(, hACC: $hACC:expr)? $(, features: $features:meta)? ) => {
        $(#[$features])?
        pub struct $virt;
        $(#[$features])?
        impl crate::private::Sealed for $virt {}

        $(#[$features])?
        periph_mrcc!(impl_rst $virt, $n, $bit $(,hRST: $hRST)?);
        $(#[$features])?
        periph_mrcc!(impl_cc  $virt, $n, $bit $(,hCC: $hCC)?);
    };

    (periph: $periph:ty, $n:expr, $bit:expr $(, hRST: $hRST:expr)? $(, hCC: $hCC:expr)? $(, hACC: $hACC:expr)? $(, features: $features:meta)? ) => {
        $(#[$features])?
        impl crate::private::Sealed for $periph {}

        $(#[$features])?
        periph_mrcc!(impl_rst $periph, $n, $bit $(,hRST: $hRST)?);
        $(#[$features])?
        periph_mrcc!(impl_cc  $periph, $n, $bit $(,hCC: $hCC)?);
    };

    (impl_rst $name:ty, $n:expr, $bit:expr) => {};
    (impl_rst $name:ty, $n:expr, $bit:expr, hRST: $hRST:expr) => {
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
    (impl_cc $name:ty, $n:expr, $bit:expr) => {};
    (impl_cc $name:ty, $n:expr, $bit:expr, hCC: $hCC:expr) => {
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
    // (impl_acc $name:ty, $n:expr, $bit:expr) => {};
    // (impl_acc $name:ty, $n:expr, $bit:expr, hACC: $hACC:expr) => {

    // };
}

pub(crate) use periph_mrcc;
