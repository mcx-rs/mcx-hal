//! General Purpose Input / Output
//!
//! # Example
//!
//! ```no_run
//! use embedded_hal::digital::StatefulOutputPin;
//! use mcx_hal::gpio::{GPIOIRQConfiguration, Output, GPIO};
//! use mcx_hal::pac;
//! use mcx_hal::port::{Port0, PortPin};
//!
//! let port0: Port0 = Port0::new(unsafe { pac::port::PORT0::instance() });
//! let mut gpio0: GPIO<0> = GPIO::new(unsafe { pac::gpio::GPIO0::instance() });
//!
//! let mut led: Output<PortPin<0, 0>> = gpio0.output(port0.p0);
//!
//! led.toggle()?;
//! led.set_high()?;
//! led.is_set_high()?;
//!
//! let mut button = gpio0.input(led.release());
//! button.is_high()?;
//! button.enable_irq(GPIOIRQConfiguration::RisingEdge);
//! button.interrupt_status()
//! ```

use crate::syscon::{PeripheralCC, PeripheralRST};
use core::convert::Infallible;
use embedded_hal as ehal;

/// GPIO Interrupt configuration.
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum GPIOIRQConfiguration {
    Disabled = 0b0000,
    RisingEdgeWithDMA = 0b0001,
    FallingEdgeWithDMA = 0b0010,
    BothEdgeWithDMA = 0b0011,
    RisingEdge = 0b0101,
    FallingEdge = 0b0110,
    BothEdge = 0b0111,
    LowWithIRQ = 0b1000,
    RisingEdgeWithIRQ = 0b1001,
    FallingEdgeWithIRQ = 0b1010,
    BothEdgeWithIRQ = 0b1011,
    HighWithIRQ = 0b1100,
}

/// GPIO Interrupt channel select.
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum GPIOIRQSelect {
    IRQ0 = 0,
    IRQ1 = 1,
}

/// GPIO
pub struct GPIO<const N: u8> {
    gpio: crate::pac::gpio::Instance<N>,
}
impl<const N: u8> GPIO<N> {
    /// Create a new GPIO instance.
    pub fn new(mut gpio: crate::pac::gpio::Instance<N>) -> Self
    where
        crate::pac::gpio::Instance<N>: crate::syscon::PeripheralRST + crate::syscon::PeripheralCC,
    {
        gpio.reset();
        gpio.enable_clock(true);
        Self { gpio }
    }

    /// Set a pin to GPIO output
    ///
    /// Use `crate::port::Port` because any Port Pin can be configured as GPIO with MUX 0.
    pub fn output<P>(&mut self, mut pin: P) -> Output<P>
    where
        P: crate::port::Port,
    {
        self.gpio.regs().PDDR().modify(|r| {
            r.set_PDD(pin.pin() as usize, true);
        });
        pin.set_mux(0);
        let index = pin.pin();
        Output {
            pin,
            gpio: self.gpio.regs(),
            index,
        }
    }

    /// Set a pin to GPIO output
    ///
    /// Use `crate::port::Port` because any Port Pin can be configured as GPIO with MUX 0.
    pub fn input<P>(&mut self, mut pin: P) -> Input<P>
    where
        P: crate::port::Port,
    {
        self.gpio.regs().PDDR().modify(|r| {
            r.set_PDD(pin.pin() as usize, false);
        });
        pin.set_mux(0);
        let index = pin.pin();
        Input {
            pin,
            gpio: self.gpio.regs(),
            index,
        }
    }
}

/// An GPIO Input pin.
pub struct Input<P> {
    pin: P,
    gpio: crate::pac::gpio::GPIO,
    index: u8,
}
impl<P> Input<P> {
    pub fn release(self) -> P {
        self.pin
    }

    pub fn enable_irq(&mut self, cfg: GPIOIRQConfiguration) {
        self.gpio
            .ICR(self.index as usize)
            .write(|r| r.set_ISF(true));
        self.gpio
            .ICR(self.index as usize)
            .write(|r| r.set_IRQC(cfg as u8));
    }

    pub fn interrupt_status(&self) -> bool {
        self.gpio.ICR(self.index as usize).read().ISF()
    }

    pub fn interrupt_configuration(&self) -> GPIOIRQConfiguration {
        unsafe { core::mem::transmute(self.gpio.ICR(self.index as usize).read().IRQC()) }
    }
}
unsafe impl<P: Send> Send for Input<P> {}
unsafe impl<P: Sync> Sync for Input<P> {}
impl<P> ehal::digital::ErrorType for Input<P> {
    type Error = Infallible;
}
impl<P> ehal::digital::InputPin for Input<P> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.gpio.PDIR().read().PDI(self.index as usize))
    }
    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!self.gpio.PDIR().read().PDI(self.index as usize))
    }
}

/// An GPIO Output pin.
pub struct Output<P> {
    pin: P,
    gpio: crate::pac::gpio::GPIO,
    index: u8,
}
impl<P> Output<P> {
    pub fn release(self) -> P {
        self.pin
    }
}
unsafe impl<P: Send> Send for Output<P> {}
unsafe impl<P: Sync> Sync for Output<P> {}
impl<P> ehal::digital::ErrorType for Output<P> {
    type Error = Infallible;
}
impl<P> ehal::digital::OutputPin for Output<P> {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.gpio
            .PSOR()
            .write(|r| r.set_PTSO(self.index as usize, true));
        Ok(())
    }
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.gpio
            .PCOR()
            .write(|r| r.set_PTCO(self.index as usize, true));
        Ok(())
    }
    fn set_state(&mut self, state: embedded_hal::digital::PinState) -> Result<(), Self::Error> {
        self.gpio
            .PDOR()
            .modify(|r| r.set_PDO(self.index as usize, state.into()));
        Ok(())
    }
}
impl<P> ehal::digital::StatefulOutputPin for Output<P> {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.gpio.PDOR().read().PDO(self.index as usize))
    }
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!self.gpio.PDOR().read().PDO(self.index as usize))
    }
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.gpio
            .PTOR()
            .write(|r| r.set_PTTO(self.index as usize, true));
        Ok(())
    }
}
