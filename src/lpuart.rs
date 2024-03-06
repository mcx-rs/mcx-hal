// use crate::pac::LPUART0 as LPUART;

// #[derive(Clone, Copy, Debug)]
// pub enum ParityMode {
//     Disabled = 0,
//     Even = 2,
//     Odd = 3,
// }

// #[derive(Clone, Copy, Debug)]
// pub enum DataBits {
//     DataBits8 = 0,
//     DataBits7 = 1,
// }

// pub enum StopBits {
//     StopBits1 = 0,
//     StopBits2 = 1,
// }

// pub struct Config {
//     pub baudrate: u32,
// }

use crate::{sealed::Sealed, HalError};
use core::marker::PhantomData;

pub struct NoDMA;

pub struct Rx<LPUART, Pin, DMAMode> {
    pin: Pin,
    _lpuart: PhantomData<LPUART>,
    _dma: PhantomData<DMAMode>,
}
pub struct Tx<LPUART, Pin, DMAMode> {
    pin: Pin,
    _lpuart: PhantomData<LPUART>,
    _dma: PhantomData<DMAMode>,
}
pub struct Serial<LPUART, TxPin, RxPin> {
    tx: Tx<LPUART, TxPin, NoDMA>,
    rx: Rx<LPUART, RxPin, NoDMA>,
}

pub trait TxPin<LPUART>: Sealed {}
pub trait RxPin<LPUART>: Sealed {}

macro_rules! lpuart {
    // wanted:
    // lpuart!(
    //   [LPUART0, (@RxPins: []), (@TxPins: [])],
    //   ...,
    // )
    (
        $([$name:ident, $instance:ident,
            @RxPins: [ $($( #[$pmetaRx:meta] )* ($rxpin:ident, $rxmux:expr),)+ ],
            @TxPins: [ $($( #[$pmetaTx:meta] )* ($txpin:ident, $txmux:expr),)+ ]],)+
    ) => {
        $(
            pub mod $name {

                use core::marker::PhantomData;
                use $crate::lpuart::{Rx, Tx, NoDMA, HalError, Serial, TxPin, RxPin};
                use $crate::pac::$instance;
                use $crate::gpio::all_gpio::*;
                use $crate::gpio::Muxed;

                $(
                    $( #[ $pmetaRx ] )*
                    impl<PinMode> RxPin<$instance> for $rxpin<Muxed<PinMode, $rxmux>> {}
                )+

                $(
                    $( #[ $pmetaTx ] )*
                    impl<PinMode> TxPin<$instance> for $txpin<Muxed<PinMode, $txmux>> {}
                )+

                impl<Pin, DMAMode> Rx<$instance, Pin, DMAMode> {}
                impl<Pin> Rx<$instance, Pin, NoDMA> {}
                impl<Pin, DMAMode> Tx<$instance, Pin, DMAMode> {}
                impl<Pin> Tx<$instance, Pin, NoDMA> {}

                impl<TP, RP> Serial<$instance, TP, RP>
                where
                    TP: TxPin<$instance>,
                    RP: RxPin<$instance>,
                {
                    fn $name(_lpuart: $instance, tx: TP, rx: RP) -> Result<Self, HalError> {
                        // TODO: check configuration and do initialize
                        Ok(Serial {
                            tx: Tx { pin: tx, _lpuart: PhantomData, _dma: PhantomData },
                            rx: Rx { pin: rx, _lpuart: PhantomData, _dma: PhantomData },
                        })
                    }
                }

                pub fn new<TP, RP>(lpuart: $instance, tx: TP, rx: RP) -> Result<Serial<$instance, TP, RP>, HalError>
                where
                    TP: TxPin<$instance>,
                    RP: RxPin<$instance>,
                {
                    Serial::$name(lpuart, tx, rx)
                }

                impl<TP, RP> Serial<$instance, TP, RP> {
                    pub fn split(self) -> (Tx<$instance, TP, NoDMA>, Rx<$instance, RP, NoDMA>) {
                        (self.tx, self.rx)
                    }

                    pub fn join(tx: Tx<$instance, TP, NoDMA>, rx: Rx<$instance, RP, NoDMA>) -> Self {
                        Self { tx, rx }
                    }

                    pub fn release(self) -> ($instance, TP, RP) {
                        // TODO: disable clock
                        let instance = unsafe { $instance::steal() };
                        (instance, self.tx.pin, self.rx.pin)
                    }
                }
            }
        )+
    };
}

// use crate::gpio::Muxed

lpuart!(
    [lpuart4, LPUART4, @RxPins: [(PIO1_8, 2),], @TxPins: [(PIO1_9, 2),]],
);
