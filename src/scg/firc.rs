use cfg_if::cfg_if;

use crate::{pac::scg::SCG, scg::SCGError};

cfg_if! {
if #[cfg(any(feature = "mcxa0", feature = "mcxa1"))] {

/// Fast Internal Reference Clock
#[derive(Clone, Copy, Default)]
#[repr(u8)]
pub enum FIRC {
    /// 48MHz FIRC clock, divided from 192MHz
    #[default]
    FIRC48M = 1,
    /// 64MHz FIRC clock
    FIRC64M = 3,
    /// 96MHz FIRC clock
    FIRC96M = 5,
    /// 192MHz FIRC clock
    FIRC192M = 7,
}

impl FIRC {
    pub fn freq(&self) -> u32 {
        match self {
            FIRC::FIRC48M => 48_000_000,
            FIRC::FIRC64M => 64_000_000,
            FIRC::FIRC96M => 96_000_000,
            FIRC::FIRC192M => 192_000_000,
        }
    }
}

} else if #[cfg(feature = "mcxa2")] {

#[derive(Clone, Copy, Default)]
#[repr(u8)]
pub enum FIRC {
    /// 45MHz FIRC clock, divided from 180MHz
    #[default]
    FIRC45M = 1,
    /// 60MHz FIRC clock
    FIRC60M = 3,
    /// 90MHz FIRC clock
    FIRC90M = 5,
    /// 180MHz FIRC clock
    FIRC180M = 7,
}

impl FIRC {
    pub fn freq(&self) -> u32 {
        match self {
            FIRC::FIRC45M => 45_000_000,
            FIRC::FIRC60M => 60_000_000,
            FIRC::FIRC90M => 90_000_000,
            FIRC::FIRC180M => 180_000_000,
        }
    }
}

}
}

impl FIRC {
    pub(crate) fn enable_firc(
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
            return Err(SCGError::FIRCErr);
        }

        Ok(())
    }

    pub(crate) fn disable_firc(scg: SCG) -> Result<(), SCGError> {
        if scg.FIRCCSR().read().FIRCSEL() {
            return Err(SCGError::Busy);
        }

        scg.FIRCCSR().modify(|r| r.set_LK(false));
        scg.FIRCCSR().modify(|r| r.set_FIRCEN(false));
        scg.FIRCCSR().modify(|r| r.set_LK(true));

        Ok(())
    }
}
