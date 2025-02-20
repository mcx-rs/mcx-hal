use core::mem::transmute;

use crate::pac::spc::Instance;

#[derive(Clone, Copy, Debug)]
pub enum SPCError {
    Busy,

    VoltageNotSupport,
}

/// Drive Voltage
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Voltage {
    UnderVoltage = 0,
    #[default]
    MidVoltage = 1,
    NormalVoltage = 2,
    OverVoltage = 3,
}

/// Drive Strength
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Strength {
    #[default]
    Low,
    Normal,
}
impl From<Strength> for bool {
    fn from(value: Strength) -> Self {
        matches!(value, Strength::Normal)
    }
}
impl From<bool> for Strength {
    fn from(value: bool) -> Self {
        if value {
            Strength::Normal
        } else {
            Strength::Low
        }
    }
}

/// System Power Control
pub struct SPC<const N: u8> {
    spc: Instance<N>,
}

impl<const N: u8> SPC<N> {
    /// Create a new SPC instance.
    pub fn new(spc: Instance<N>) -> Self {
        Self { spc }
    }

    /// Setup Active mode CoreLDO.
    pub fn set_active_core_ldo(
        &mut self,
        voltage: Voltage,
        strength: Strength,
    ) -> Result<(), SPCError> {
        if self.busy() {
            return Err(SPCError::Busy);
        }

        // TODO: Check Bandgap

        // TODO: To set to low drive strength, all LVDs/HVDs must be disabled previously

        if self.active_core_ldo_voltage() != voltage {
            self.set_active_core_ldo_strength(Strength::Normal);
            self.set_active_core_ldo_voltage(voltage);
        }
        self.set_active_core_ldo_strength(strength);
        Ok(())
    }

    /// Setup SRAM operate voltage.
    ///
    /// `Voltage::UnderVoltage` is not supported.
    /// `Voltage::MidVoltage` -> 1.0V
    /// `Voltage::NormalVoltage` -> 1.1V
    /// `Voltage::OverVoltage` -> 1.2V
    pub fn set_sram_operate_voltage(&mut self, voltage: Voltage) -> Result<(), SPCError> {
        if voltage == Voltage::UnderVoltage {
            return Err(SPCError::VoltageNotSupport);
        }

        self.spc.regs().SRAMCTL().write(|r| {
            r.set_VSM(voltage as u8);
            r.set_REQ(true);
        });
        while !self.spc.regs().SRAMCTL().read().ACK() {}
        self.spc.regs().SRAMCTL().modify(|r| r.set_REQ(false));

        Ok(())
    }

    /// Get current Active mode CoreLDO voltage.
    #[inline(always)]
    pub fn active_core_ldo_voltage(&self) -> Voltage {
        // This register read can not read value outside `Voltage` unless it broken
        unsafe { transmute(self.spc.regs().ACTIVE_CFG().read().CORELDO_VDD_LVL()) }
    }

    #[inline(always)]
    fn set_active_core_ldo_voltage(&mut self, voltage: Voltage) {
        self.spc
            .regs()
            .ACTIVE_CFG()
            .modify(|r| r.set_CORELDO_VDD_LVL(voltage as u8));
        while self.busy() {}
    }

    /// Get current Active mode CoreLDO drive strength
    #[inline(always)]
    pub fn active_core_ldo_strength(&self) -> Strength {
        self.spc.regs().ACTIVE_CFG().read().CORELDO_VDD_DS().into()
    }

    #[inline(always)]
    fn set_active_core_ldo_strength(&mut self, strength: Strength) {
        self.spc
            .regs()
            .ACTIVE_CFG()
            .modify(|r| r.set_CORELDO_VDD_DS(strength.into()));
    }

    /// Is current SPC busy?
    #[inline(always)]
    pub fn busy(&self) -> bool {
        self.spc.regs().SC().read().BUSY()
    }
}
