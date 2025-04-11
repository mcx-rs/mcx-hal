//! General Purpose Input / Output
//!
//! # Example
//!
//! ```rust
//! let port0: Port0 = Port0::new(unsafe { pac::port::PORT0::instance() });
//! let mut gpio0 = GPIO::new(unsafe { pac::gpio::GPIO0::instance() });
//! let led = gpio0.output(port0.p0);
//!
//! // mcx-hal methods
//! led.set();
//! led.clear();
//! led.toggle();
//!
//! // embedded-hal methods
//! led.set_high().unwrap();
//! led.set_low().unwrap();
//! led.toggle().unwrap();
//!
//! // use interrupt
//! let btn = gpio0.input(port0.p1);
//! btn.set_interrupt_config(GPIOIRQConfig::InterruptFallingEdge);
//! // clear interrupt flag
//! btn.clear_interrupt_flag();
//! ```

use crate::{
    consts::{Const, Unsigned},
    pac::gpio::Instance,
    port::Port,
    syscon::{PeripheralCC, PeripheralRST},
};
use core::convert::Infallible;
use eh1;

/// GPIO Pin direction.
#[derive(Clone, Copy)]
pub enum Direction {
    Input,
    Output,
}

/// GPIO interrupt configuration.
#[derive(Clone, Copy, Default)]
pub enum GPIOIRQConfig {
    #[default]
    Disabled = 0,

    DMARisingEdge = 1,
    DMAFallingEdge = 2,
    DMAEitherEdge = 3,

    FlagRisingEdge = 5,
    FlagFallingEdge = 6,
    FlagEitherEdge = 7,

    InterruptLogicZero = 8,
    InterruptRisingEdge = 9,
    InterruptFallingEdge = 10,
    InterruptEitherEdge = 11,
    InterruptLogicOne = 12,

    ActiveHighTriggerOutputEnable = 13,
    ActiveLowTriggerOutputEnable = 14,
}

/// GPIO driver.
pub struct GPIO<const N: u8> {
    gpio: Instance<N>,
}

/// GPIO digital output pin.
pub struct Output<P>
where
    P: Port,
{
    pin: P,
    gpio: crate::pac::gpio::GPIO,
}

/// GPIO digital input pin.
pub struct Input<P>
where
    P: Port,
{
    pin: P,
    gpio: crate::pac::gpio::GPIO,
}

impl<const N: u8> GPIO<N> {
    /// Create a new GPIO driver.
    pub fn new(mut gpio: Instance<N>) -> Self
    where
        Instance<N>: PeripheralRST + PeripheralCC,
    {
        gpio.reset();
        gpio.enable_clock(true);
        Self { gpio }
    }

    /// Set a pin into GPIO digital input pin.
    /// See also [`Input`].
    pub fn input<P>(&mut self, mut pin: P) -> Input<P>
    where
        P: Port<PORT = Const<N>>,
    {
        pin.set_mux(0);
        self.regs()
            .PDDR()
            .modify(|r| r.set_PDD(P::PIN::USIZE, false));

        Input {
            pin,
            gpio: self.regs(),
        }
    }

    /// Set a pin into GPIO digital output pin.
    /// See also [`Output`].
    pub fn output<P>(&mut self, mut pin: P) -> Output<P>
    where
        P: Port<PORT = Const<N>>,
    {
        pin.set_mux(0);
        self.regs()
            .PDDR()
            .modify(|r| r.set_PDD(P::PIN::USIZE, true));

        Output {
            pin,
            gpio: self.regs(),
        }
    }

    fn regs(&self) -> crate::pac::gpio::GPIO {
        self.gpio.regs()
    }
}
impl<P: Port> Output<P> {
    /// Get current GPIO pin mask.
    pub const fn mask(&self) -> u32 {
        1 << P::PIN::USIZE as u32
    }

    /// Set GPIO pin output.
    pub fn set(&self) {
        self.gpio.PSOR().write(|r| r.0 = self.mask());
    }

    /// Clear GPIO pin output.
    pub fn clear(&self) {
        self.gpio.PCOR().write(|r| r.0 = self.mask());
    }

    /// Toggle GPIO pin output.
    pub fn toggle(&self) {
        self.gpio.PTOR().write(|r| r.0 = self.mask());
    }

    /// Return `true` if GPIO pin is set.
    pub fn is_set(&self) -> bool {
        self.gpio.PDR(P::PIN::USIZE).read().0 != 0
    }

    /// Return `true` if GPIO pin is cleared.
    pub fn is_clear(&self) -> bool {
        self.gpio.PDR(P::PIN::USIZE).read().0 == 0
    }

    /// Release a GPIO pin.
    pub fn release(self) -> P {
        self.pin
    }

    /// Access the raw pin.
    pub fn pin(&self) -> &P {
        &self.pin
    }

    /// Access the mutable raw pin.
    pub fn mut_pin(&mut self) -> &mut P {
        &mut self.pin
    }
}
impl<P: Port> Input<P> {
    /// Get current GPIO pin mask.
    pub const fn mask(&self) -> u32 {
        1 << P::PIN::USIZE as u32
    }

    /// Return `true` if GPIO pin is set.
    pub fn is_set(&self) -> bool {
        self.gpio.PDR(P::PIN::USIZE).read().0 != 0
    }

    /// Return `true` if GPIO pin is cleared.
    pub fn is_clear(&self) -> bool {
        self.gpio.PDR(P::PIN::USIZE).read().0 == 0
    }

    /// Set GPIO pin's interrupt configuration.
    /// Use [`GPIOIRQ::Disabled`] to disable GPIO pin's interrupt.
    pub fn set_interrupt_config(&mut self, irq: GPIOIRQConfig) {
        self.gpio.ICR(P::PIN::USIZE).write(|r| {
            r.set_ISF(true);
            r.set_IRQC(irq as u8);
        });
    }

    /// Read GPIO pin's interrupt flag.
    #[inline]
    pub fn get_interrupt_flag(&self) -> bool {
        self.gpio.ICR(P::PIN::USIZE).read().ISF()
    }

    /// Clear GPIO pin's interrupt flag by setting bit.
    pub fn clear_interrupt_flag(&mut self) {
        self.gpio.ICR(P::PIN::USIZE).modify(|r| r.set_ISF(true));
    }

    /// Release a GPIO pin.
    pub fn release(self) -> P {
        self.pin
    }

    /// Access the raw pin.
    pub fn pin(&self) -> &P {
        &self.pin
    }

    /// Access the mutable raw pin.
    pub fn mut_pin(&mut self) -> &mut P {
        &mut self.pin
    }
}

impl<P: Port> eh1::digital::ErrorType for Input<P> {
    type Error = Infallible;
}
impl<P: Port> eh1::digital::ErrorType for Output<P> {
    type Error = Infallible;
}
impl<P: Port> eh1::digital::InputPin for Input<P> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.is_set())
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.is_clear())
    }
}
impl<P: Port> eh1::digital::OutputPin for Output<P> {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set();
        Ok(())
    }
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.clear();
        Ok(())
    }
    fn set_state(&mut self, state: eh1::digital::PinState) -> Result<(), Self::Error> {
        self.gpio
            .PDR(P::PIN::USIZE)
            .write(|r| r.set_PD(state.into()));
        Ok(())
    }
}
impl<P: Port> eh1::digital::StatefulOutputPin for Output<P> {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.is_set())
    }
    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.is_clear())
    }
    fn toggle(&mut self) -> Result<(), Self::Error> {
        Output::<P>::toggle(self);
        Ok(())
    }
}
