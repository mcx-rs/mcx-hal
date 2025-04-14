//! Phase-locked loop.
//!
//! Use [`PllConfig`] to configure a PLL instance (SPLL or APLL).
//!
//! # Normal mode
//! This is current only supported PLL mode.
//! Normal mode PLL contains three dividers:
//! - Predivider N (from 1 to 2^8-1)
//! - Feedback-divider M (from 1 to 2^16-1)
//! - Postdivider P * 2 (from 1 to 2^5-1)
//!
//! # Spread Spectrum and Fractional Modes
//! Spread Spectrum and Fractional modes are not supported right now.

use super::SCGError;
use crate::pac::scg::SCG;

/// Pll Source.
#[derive(Clone, Copy, Default)]
pub enum PllSource {
    #[default]
    SOSC = 0,
    FIRC = 1,
    ROSC = 2,
    SIRC = 3,
}

/// Pll Configuration.
///
/// In RM, NXP does NOT recommend disable predivider N.
#[derive(Clone, Copy)]
pub struct PllConfig {
    pub source: PllSource,
    pub n: Option<u8>,
    pub m: u16,
    pub p: Option<u8>,
    pub p2: bool,
}

impl PllSource {
    pub const fn valid(&self) -> bool {
        match self {
            _ => true,
        }
    }
}

impl PllConfig {
    /// Get the CCO frequency.
    ///
    /// CCO frequency should between 275MHz - 550MHz according to RM.
    #[inline]
    pub const fn freq_cco(&self, source_clk: u32) -> u32 {
        match self.n {
            Some(n) => self.m as u32 / n as u32 * source_clk,
            None => self.m as u32 * source_clk,
        }
    }

    #[inline]
    pub const fn freq_out(&self, source_clk: u32) -> u32 {
        match self.p {
            Some(p) => self.freq_cco(source_clk) / 2 / p as u32,
            None => self.freq_cco(source_clk),
        }
    }

    #[inline]
    pub const fn freq(&self, source_clk: u32) -> u32 {
        self.freq_out(source_clk)
    }

    pub const fn valid(&self) -> bool {
        if self.m == 0 {
            return false;
        }
        if self.p.is_some() {
            if self.p.unwrap() == 0 {
                return false;
            }
            if self.p.unwrap() >= 0b11111 {
                return false;
            }
        }
        if self.n.is_some() {
            if self.n.unwrap() == 0 {
                return false;
            }
        }

        self.source.valid()
    }

    pub(crate) fn enable_spll(scg: SCG, config: PllConfig, stop_en: bool) -> Result<(), SCGError> {
        // enable LDO
        scg.LDOCSR().modify(|r| r.set_LDOEN(true));

        // power off SPLL and disable SPLL clock
        scg.SPLLCSR().modify(|r| {
            r.set_SPLLPWREN(false);
            r.set_SPLLCLKEN(false);
        });

        scg.SPLLCTRL().modify(|r| {
            r.set_SOURCE(config.source as u8);
            r.set_SELI(config.seli());
            r.set_SELP(config.selp());
            r.set_SELR(0);
            r.set_BYPASSPOSTDIV2(config.p2);
        });

        match config.n {
            Some(n) => scg.SPLLNDIV().write(|r| {
                r.set_NDIV(n);
                r.set_NREQ(true);
            }),
            None => scg.SPLLCTRL().modify(|r| r.set_BYPASSPREDIV(true)),
        }
        scg.SPLLMDIV().write(|r| {
            r.set_MDIV(config.m);
            r.set_MREQ(true);
        });
        match config.p {
            Some(p) => scg.SPLLPDIV().write(|r| {
                r.set_PDIV(p);
                r.set_PREQ(true);
            }),
            None => scg.SPLLCTRL().modify(|r| r.set_BYPASSPOSTDIV(true)),
        }

        scg.SPLLCSR().modify(|r| {
            r.set_SPLLPWREN(true);
            r.set_SPLLCLKEN(true);
            r.set_SPLLSTEN(stop_en);
        });

        while !scg.SPLLCSR().read().SPLL_LOCK() {}

        Ok(())
    }

    pub(crate) fn disable_spll(scg: SCG) -> Result<(), SCGError> {
        if scg.SPLLCSR().read().SPLLSEL() {
            return Err(SCGError::SPLLBusy);
        }

        scg.SPLLCSR().modify(|r| {
            r.set_SPLLPWREN(false);
            r.set_SPLLCLKEN(false);
        });

        Ok(())
    }

    const fn selp(&self) -> u8 {
        let a = self.m / 4 + 1;
        if a >= 31 {
            31
        } else {
            a as u8
        }
    }

    const fn seli(&self) -> u8 {
        let a = match self.m {
            0 => unreachable!(),
            1..122 => 2 * (self.m / 4) + 3,
            122..8000 => 8000 / self.m,
            _ => 1,
        };

        if a >= 63 {
            63
        } else {
            a as u8
        }
    }
}
