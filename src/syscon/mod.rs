//! System Controller.
//!
//! This module provides the peripheral control for NXP MCX MCUs.

use crate::private::Sealed;

/// Peripheral reset trait.
pub trait PeripheralRST: Sealed {
    /// Assert or release the reset signal.
    fn assert_reset(&mut self, release: bool);

    /// Reset the peripheral.
    fn reset(&mut self) {
        self.assert_reset(true);
        self.assert_reset(false);
    }
}

/// Peripheral clock control trait.
pub trait PeripheralCC: Sealed {
    /// Enable or disable the peripheral clock.
    fn enable_clock(&mut self, enable: bool);
}

/// Peripheral enable trait.
pub trait PeripheralEn: Sealed {
    /// Enable or disable the peripheral.
    fn enable(enable: bool);
}

#[cfg_attr(feature = "mcxa0", path = "device/a0.rs")]
#[cfg_attr(feature = "mcxa1", path = "device/a1.rs")]
#[cfg_attr(feature = "mcxa2", path = "device/a2.rs")]
mod device;
pub use device::*;

mod mrcc;
use mrcc::periph_mrcc;
