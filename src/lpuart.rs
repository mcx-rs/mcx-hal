//! # LPUART Driver

/// LPUART Configuration
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Config {
    pub baudrate: u32,
}

impl Config {
    /// check if the LPUART Configuration is valid
    pub fn is_valid(&self, _clock: u32) -> bool {
        todo!()
    }
}

/// LPUART Pins
pub struct Pins<const N: u8, Tx, Rx>
where
    Tx: LpUartTxPin<N>,
    Rx: LpUartRxPin<N>,
{
    pub tx: Tx,
    pub rx: Rx,
}

/// LPUART
pub struct LpUart<PINS, const N: u8> {
    pub pins: PINS,
}

impl<TXPIN, RXPIN, const N: u8> LpUart<Pins<N, TXPIN, RXPIN>, N>
where
    TXPIN: LpUartTxPin<N>,
    RXPIN: LpUartRxPin<N>,
{
    fn get_register_block(&self) -> &'static crate::pac::lpuart0::RegisterBlock {
        match N {
            0 => unsafe { &*crate::pac::LPUART0::ptr() },
            1 => unsafe { &*crate::pac::LPUART1::ptr() },
            2 => unsafe { &*crate::pac::LPUART2::ptr() },
            3 => unsafe { &*crate::pac::LPUART3::ptr() },
            4 => unsafe { &*crate::pac::LPUART4::ptr() },
            5 => unsafe { &*crate::pac::LPUART5::ptr() },
            6 => unsafe { &*crate::pac::LPUART6::ptr() },
            7 => unsafe { &*crate::pac::LPUART7::ptr() },
            8 => unsafe { &*crate::pac::LPUART8::ptr() },
            9 => unsafe { &*crate::pac::LPUART9::ptr() },
            _ => unreachable!(),
        }
    }
}

pub(crate) trait LpUartInstance {
    /// do not move instance out of driver scope
    fn get_instance() -> Self;
}

/// LPUART Tx Pin
pub trait LpUartTxPin<const N: u8> {}
/// LPUART Rx Pin
pub trait LpUartRxPin<const N: u8> {}

// TODO: implement LpUartTxPin and LpUartRxPin in macro
macro_rules! lpuart {
    ($index:expr) => {
        paste::paste! {
            pub mod [< lpuart $index >] {
                use $crate::pac::LPUART4 as LPUART;

                use $crate::HalError;
                use $crate::lpuart::{LpUart, Pins, LpUartTxPin, LpUartRxPin, Config};

                pub fn new<TXPIN, RXPIN>(
                    _instance: LPUART,
                    txpin: TXPIN,
                    rxpin: RXPIN,
                    config: Config,
                    clock: u32,
                ) -> Result<LpUart<Pins<4, TXPIN, RXPIN>, 4>, HalError>
                where
                    TXPIN: LpUartTxPin<4>,
                    RXPIN: LpUartRxPin<4>,
                {
                    if !config.is_valid(clock) {
                        return Err(HalError::InvalidConfig);
                    }

                    Ok(LpUart {
                        pins: Pins {
                            tx: txpin,
                            rx: rxpin,
                        }
                    })
                }
            }
        }
    };
}

pub mod lpuart4_test {
    use crate::lpuart::{Config, LpUart, LpUartInstance, LpUartRxPin, LpUartTxPin, Pins};
    use crate::pac::LPUART4 as LPUART;
    use crate::HalError;

    pub fn new<TXPIN, RXPIN>(
        instance: LPUART,
        txpin: TXPIN,
        rxpin: RXPIN,
        config: Config,
        clock: u32,
    ) -> Result<LpUart<Pins<4, TXPIN, RXPIN>, 4>, HalError>
    where
        TXPIN: LpUartTxPin<4>,
        RXPIN: LpUartRxPin<4>,
    {
        if !config.is_valid(clock) {
            return Err(HalError::InvalidConfig);
        }

        let flexcomm = unsafe { crate::pac::LP_FLEXCOMM4::steal() };
        let lpuart = unsafe { crate::pac::LPUART4::steal() };

        flexcomm.pselid().write(|w| w.persel().uart());

        let mut osr: u8 = 0;
        let mut sbr: u16 = 0;
        let mut osr_temp: u8 = 0;
        let mut sbr_temp: u16 = 0;
        let mut baud_diff = config.baudrate;

        osr_temp = 4;
        while osr_temp <= 32 {
            sbr_temp = ((clock * 10 / (config.baudrate * osr_temp as u32) + 5) / 10) as u16;
            if sbr_temp == 0 {
                sbr_temp = 1;
            } else if sbr_temp > 2 ^ 13 - 1 {
                sbr_temp = 2 ^ 13 - 1;
            }
            let calculated_baud = clock / (osr_temp as u32 * sbr_temp as u32);
            let temp_diff = if calculated_baud > config.baudrate {
                calculated_baud - config.baudrate
            } else {
                config.baudrate - calculated_baud
            };
            if temp_diff <= baud_diff {
                baud_diff = temp_diff;
                osr = osr_temp;
                sbr = sbr_temp;
            }
        }

        if baud_diff > config.baudrate / 100 * 3 {
            return Err(HalError::InvalidConfig);
        }

        Ok(LpUart {
            pins: Pins {
                tx: txpin,
                rx: rxpin,
            },
        })
    }
}

lpuart!(4);
