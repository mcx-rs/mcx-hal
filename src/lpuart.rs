use core::marker::PhantomData;

pub struct ClockConfig {}

#[derive(Clone, Copy, Debug)]
pub struct Config {}

pub struct NoDMA;

pub struct TXD<INSTANCE, Pin, DMAMode> {
    pin: Pin,
    _instance: PhantomData<INSTANCE>,
    _dma: PhantomData<DMAMode>,
}

pub struct RXD<INSTANCE, Pin, DMAMode> {
    pin: Pin,
    _instance: PhantomData<INSTANCE>,
    _dma: PhantomData<DMAMode>,
}

pub struct LpUart<INSTANCE, TxPin, RxPin> {
    rxd: RXD<INSTANCE, RxPin, NoDMA>,
    txd: TXD<INSTANCE, TxPin, NoDMA>,
}

pub mod lpuart4 {
    use crate::lpuart::{Config, LpUart};
    use crate::pac::LPUART4;
    use crate::HalError;

    pub fn new<INSTANCE, TP, RP>(
        _instance: LPUART4,
        txpin: TP,
        rxpin: RP,
        config: Config,
    ) -> Result<LpUart<INSTANCE, TP, RP>, HalError> {
        todo!()
    }
}
