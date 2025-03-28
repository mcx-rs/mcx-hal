//！Fast Internal Reference Clock

use crate::{pac::scg::SCG, scg::SCGError};

#[cfg(any(feature = "mcxa0", feature = "mcxa1"))]
#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum FIRC {
    #[default]
    FIRC48M = 1,
    FIRC64M = 3,
    FIRC96M = 5,
    FIRC192M = 7,
}

#[cfg(feature = "mcxa2")]
#[derive(Debug, Clone, Copy, Default)]
#[repr(u8)]
pub enum FIRC {
    #[default]
    FIRC45M = 1,
    FIRC60M = 3,
    FIRC90M = 5,
    FIRC180M = 7,
}

impl FIRC {
    pub const fn freq(&self) -> u32 {
        #[cfg(any(feature = "mcxa0", feature = "mcxa1"))]
        match self {
            Self::FIRC48M => 48_000_000,
            Self::FIRC64M => 64_000_000,
            Self::FIRC96M => 96_000_000,
            Self::FIRC192M => 192_000_000,
        }
        #[cfg(feature = "mcxa2")]
        match self {
            Self::FIRC45M => 45_000_000,
            Self::FIRC60M => 60_000_000,
            Self::FIRC90M => 90_000_000,
            Self::FIRC180M => 180_000_000,
        }
    }

    pub(crate) fn enable(
        scg: SCG,
        firc: FIRC,
        stop_en: bool,
        fclk_en: bool,
        sclk_en: bool,
    ) -> Result<(), SCGError> {
        scg.FIRCCFG().write(|r| r.set_FREQ_SEL(firc as u8));

        scg.FIRCCSR().modify(|r| r.set_LK(false));
        scg.FIRCCSR().modify(|r| {
            r.set_FIRCEN(true);
            r.set_FIRCSTEN(stop_en);
            r.set_FIRC_SCLK_PERIPH_EN(sclk_en);
            r.set_FIRC_FCLK_PERIPH_EN(fclk_en);
        });
        scg.FIRCCSR().modify(|r| r.set_LK(true));

        while !scg.FIRCCSR().read().FIRCVLD() {}
        if scg.FIRCCSR().read().FIRCERR() {
            return Err(SCGError::FIRCError);
        }

        Ok(())
    }

    pub(crate) fn disable(scg: SCG) -> Result<(), SCGError> {
        if scg.FIRCCSR().read().FIRCSEL() {
            return Err(SCGError::FIRCBusy);
        }

        scg.FIRCCSR().modify(|r| r.set_LK(false));
        scg.FIRCCSR().modify(|r| r.set_FIRCEN(false));
        scg.FIRCCSR().modify(|r| r.set_LK(true));

        Ok(())
    }
}
