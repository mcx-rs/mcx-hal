//! Low Power Universal Asynchronous Receiver / Transmitter

use crate::{
    pac::{self, lpuart::Instance},
    port::{
        consts::Const,
        lpuart::{prepare, Pin, RXD, TXD},
    },
    syscon::{PeripheralCC, PeripheralRST},
};

/// LPUART Errors
#[derive(Clone, Copy, Debug)]
pub enum LpUartError {
    BaudRateNotSupport,
}

/// Baud Rate resolver for LPUART.
pub struct BaudRate {
    pub osr: u8,
    pub sbr: u16,
    pub bothedge: bool,
}
impl BaudRate {
    pub const fn new(source_clk_hz: u32, target_bps: u32) -> Result<Self, LpUartError> {
        let mut osr = 0u8;
        let mut sbr = 0u16;

        let mut osr_tmp;
        let mut sbr_tmp;
        let mut baud_diff = target_bps;

        osr_tmp = 4;
        while osr_tmp <= 32 {
            sbr_tmp = (source_clk_hz * 2 / (target_bps * osr_tmp as u32 + 1) / 2) as u16;
            if sbr_tmp == 0 {
                sbr_tmp = 1;
            } else if sbr_tmp > 0x1FFF {
                sbr_tmp = 0x1FFF;
            }

            let calculated_bps = source_clk_hz / (osr_tmp as u32 * sbr_tmp as u32);
            let diff = if calculated_bps > target_bps {
                calculated_bps - target_bps
            } else {
                target_bps - calculated_bps
            };
            if diff < baud_diff {
                baud_diff = diff;
                osr = osr_tmp;
                sbr = sbr_tmp;
            }

            osr_tmp += 1;
        }

        if baud_diff > target_bps / 100 * 3 {
            return Err(LpUartError::BaudRateNotSupport);
        }

        Ok(Self {
            osr,
            sbr,
            bothedge: osr > 3 && osr < 8,
        })
    }

    pub const fn value(&self, source_clk_hz: u32) -> u32 {
        source_clk_hz / (self.osr as u32 * self.sbr as u32)
    }
}

/// LPUART Pins, only contains TXD and RXD
pub struct Pins<TX, RX>
where
    TX: Pin<Signal = TXD>,
    RX: Pin<Signal = RXD, Module = TX::Module>,
{
    pub tx: TX,
    pub rx: RX,
}

/// LPUART instance
pub struct LpUart<const N: u8, PINS> {
    lpuart: Instance<N>,
    pins: PINS,
}

impl<const N: u8, TX, RX> LpUart<N, Pins<TX, RX>>
where
    TX: Pin<Signal = TXD, Module = Const<N>>,
    RX: Pin<Signal = RXD, Module = Const<N>>,
{
    /// Create a new LPUART instance with given TX and RX pins
    pub fn new(mut lpuart: Instance<N>, mut pins: Pins<TX, RX>) -> Self
    where
        Instance<N>: PeripheralCC + PeripheralRST,
    {
        lpuart.reset();
        lpuart.enable_clock(true);

        prepare(&mut pins.tx);
        prepare(&mut pins.rx);

        let mut ret = Self { lpuart, pins };
        ret.reset();
        ret
    }
}
impl<const N: u8> LpUart<N, ()> {
    /// Create a new LPUART instance without pins
    pub fn without_pins(mut lpuart: Instance<N>) -> Self
    where
        Instance<N>: crate::syscon::PeripheralCC + crate::syscon::PeripheralRST,
    {
        lpuart.reset();
        lpuart.enable_clock(true);

        let mut ret = Self { lpuart, pins: () };
        ret.reset();
        ret
    }
}
impl<const N: u8, PINS> LpUart<N, PINS> {
    /// Reset LPUART to default and release instance and pins.
    pub fn release(mut self) -> (Instance<N>, PINS) {
        self.reset();
        (self.lpuart, self.pins)
    }

    /// Reset LPUART to default.
    pub fn reset(&mut self) {
        self.lpuart.regs().GLOBAL().modify(|r| r.set_RST(true));
        self.lpuart.regs().GLOBAL().modify(|r| r.set_RST(false));
    }

    /// Return the parity mode.
    pub fn parity(&self) -> Option<ParityMode> {
        let ctrl = self.lpuart.regs().CTRL().read();
        if !ctrl.PE() {
            return ParityMode::NONE;
        }
        if ctrl.PT() {
            ParityMode::ODD
        } else {
            ParityMode::EVEN
        }
    }

    /// Configure LPUART in a disabled status.
    pub fn configure<R>(&mut self, f: impl FnOnce(&mut Disabled<N>) -> R) -> R {
        let mut disabled = Disabled::new(&self.lpuart);
        f(&mut disabled)
    }

    /// Set LPUART transfer/receive enable.
    pub fn set_enable(&mut self, direction: Direction, enable: bool) {
        self.lpuart.regs().CTRL().modify(|r| match direction {
            Direction::RX => r.set_RE(enable),
            Direction::TX => r.set_TE(enable),
        });
    }

    /// Return if LPUART transfer enabled.
    pub fn is_tx_enable(&self) -> bool {
        self.lpuart.regs().CTRL().read().TE()
    }

    /// Return if LPUART receive enabled.
    pub fn is_rx_enable(&self) -> bool {
        self.lpuart.regs().CTRL().read().RE()
    }

    /// Write single byte without any check.
    /// # Safety
    /// Please check LPUART's status before write or read.
    pub unsafe fn write_byte(&mut self, data: u8) {
        self.regs().DATA().write(|r| r.0 = data as u32);
    }

    /// Read single byte without any check.
    /// # Safety
    /// Please check LPUART's status before write or read.
    pub unsafe fn read_byte(&self) -> u8 {
        self.regs().DATA().read().0 as u8
    }

    fn regs(&self) -> pac::lpuart::LPUART {
        self.lpuart.regs()
    }
}

#[derive(Clone, Copy)]
pub enum LpUartFifoWaterMark {}

pub struct Disabled<'a, const N: u8> {
    lpuart: &'a Instance<N>,
    te: bool,
    re: bool,
}
impl<const N: u8> Drop for Disabled<'_, N> {
    fn drop(&mut self) {
        self.lpuart.regs().CTRL().modify(|r| {
            r.set_TE(self.te);
            r.set_RE(self.re);
        });
    }
}
impl<'a, const N: u8> Disabled<'a, N> {
    fn new(lpuart: &'a Instance<N>) -> Self {
        let re = lpuart.regs().CTRL().read().RE();
        let te = lpuart.regs().CTRL().read().TE();
        lpuart.regs().CTRL().modify(|r| {
            r.set_TE(false);
            r.set_RE(false);
        });

        // TODO: flush fifo

        Self { lpuart, te, re }
    }

    pub fn set_baud(&mut self, baud: &BaudRate) {
        self.lpuart.regs().BAUD().modify(|r| {
            r.set_BOTHEDGE(baud.bothedge);
            r.set_OSR(baud.osr - 1);
            r.set_SBR(baud.sbr);
        });
    }

    pub fn set_parity(&mut self, parity: Option<ParityMode>) {
        self.lpuart.regs().CTRL().modify(|r| {
            r.set_PE(parity.is_some());
            r.set_M(parity.is_some());
            r.set_PT(parity.unwrap_or(ParityMode::Even) as u32 == 1);
        });
    }
}

/// LPUART Parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum ParityMode {
    Even = 0,
    Odd = 1,
}
impl ParityMode {
    pub const NONE: Option<ParityMode> = None;
    pub const EVEN: Option<ParityMode> = Some(Self::Even);
    pub const ODD: Option<ParityMode> = Some(Self::Odd);
}

/// LPUART Direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    TX,
    RX,
}

impl eio06::Error for LpUartError {
    fn kind(&self) -> eio06::ErrorKind {
        match self {
            _ => eio06::ErrorKind::Other,
        }
    }
}
impl<const N: u8, PINS> eio06::ErrorType for LpUart<N, PINS> {
    type Error = LpUartError;
}
impl<const N: u8, PINS> eio06::WriteReady for LpUart<N, PINS> {
    fn write_ready(&mut self) -> Result<bool, Self::Error> {
        let stat = self.regs().STAT().read();
        Ok(stat.TDRE())
    }
}
impl<const N: u8, PINS> eio06::ReadReady for LpUart<N, PINS> {
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        let stat = self.regs().STAT().read();
        Ok(stat.RDRF())
    }
}
impl<const N: u8, PINS> eio06::Write for LpUart<N, PINS> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let mut num_written = 0;

        // Block until write 1 byte
        while !self.regs().STAT().read().TDRE() {}
        self.regs().DATA().write(|r| r.0 = buf[num_written] as u32);
        num_written += 1;

        while num_written < buf.len() {
            if !self.regs().STAT().read().TDRE() {
                break;
            }

            self.regs().DATA().write(|r| r.0 = buf[num_written] as u32);
            num_written += 1;
        }

        Ok(num_written)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        while !self.regs().STAT().read().TC() {}
        Ok(())
    }
}
impl<const N: u8, PINS> eio06::Read for LpUart<N, PINS> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let mut num_read = 0;

        loop {
            let data = self.regs().DATA().read();
            if !data.RXEMPT() {
                buf[num_read] = data.0 as u8;
                break;
            }
        }
        num_read += 1;

        while num_read < buf.len() {
            let data = self.regs().DATA().read();
            if data.RXEMPT() {
                break;
            }

            buf[num_read] = data.0 as u8;
            num_read += 1;
        }

        Ok(num_read)
    }
}
