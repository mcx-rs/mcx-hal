//! System Crystal Oscillator Clock (SOSC)

use crate::{pac::scg::SCG, scg::SCGError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SOSC {
    RefClock(u32),
    Oscillator(u32),
}

impl SOSC {
    /// Return SOSC's frequency.
    pub const fn freq(&self) -> u32 {
        match self {
            SOSC::RefClock(f) => *f,
            SOSC::Oscillator(f) => *f,
        }
    }

    pub(crate) fn enable(scg: SCG, sosc: SOSC, stopen: bool) -> Result<(), SCGError> {
        #[cfg(feature = "mcxa2")]
        {
            scg.LDOCSR().modify(|r| r.set_LDOEN(true));
            while !scg.LDOCSR().read().VOUT_OK() {}
        }

        let range = sosc.range()?;
        scg.SOSCCFG().write(|r| {
            r.set_EREFS(matches!(sosc, SOSC::Oscillator(_)));
            r.set_RANGE(range);
        });
        scg.SOSCCSR().modify(|r| r.set_LK(false));
        scg.SOSCCSR().modify(|r| {
            r.set_SOSCEN(true);
            r.set_SOSCSTEN(stopen);
            r.set_LK(true);
        });

        while !scg.SOSCCSR().read().SOSCVLD() {}
        if scg.SOSCCSR().read().SOSCERR() {
            return Err(SCGError::SOSCError);
        }

        Ok(())
    }

    pub(crate) fn disable(scg: SCG) -> Result<(), SCGError> {
        if scg.SOSCCSR().read().SOSCSEL() {
            return Err(SCGError::SOSCBusy);
        }

        // Do NOT stop LDO here, because we don't known if PLLs are using LDO.

        scg.SOSCCSR().modify(|r| r.set_LK(false));
        scg.SOSCCSR().modify(|r| {
            r.set_SOSCEN(false);
            r.set_LK(true);
        });

        Ok(())
    }

    const fn range(&self) -> Result<u8, SCGError> {
        match self.freq() {
            8_000_000..16_000_000 => Ok(0),
            16_000_000..25_000_000 => Ok(1),
            25_000_000..40_000_000 => Ok(2),
            40_000_000..50_000_000 => Ok(3),
            _ => Err(SCGError::OutOfRange),
        }
    }
}
