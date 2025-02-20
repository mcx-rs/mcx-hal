use super::SCGError;

#[derive(Clone, Copy, Default)]
pub enum PllSource {
    #[default]
    SOSC = 0,
    FIRC = 1,
    ROSC = 2,
    SIRC = 3,
}
impl PllSource {
    pub const fn valid(&self) -> bool {
        match self {
            #[cfg(feature = "mcxn")]
            PllSource::ROSC | PllSource::SIRC => false,

            _ => true,
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct PllConfig {
    pub source: PllSource,
    pub n: u8,
    pub p: u8,
    pub m: u16,
}
impl PllConfig {
    const fn valid(&self) -> bool {
        self.source.valid()
    }

    const fn selp(&self) -> u8 {
        let a: u16 = self.m >> 2 + 1;
        if a >= 31 {
            31
        } else {
            a as u8
        }
    }
    const fn seli(&self) -> u8 {
        let a: u16 = {
            if self.m >= 8000 {
                1
            } else if 122 <= self.m && self.m < 8000 {
                8000 / self.m
            } else {
                2 * (self.m >> 2) + 3
            }
        };

        if a >= 63 {
            63
        } else {
            a as u8
        }
    }
}

#[cfg(feature = "mcxa2")]
pub(crate) fn enable_spll(
    scg: crate::pac::scg::SCG,
    config: PllConfig,
    source_clk_freq: u32,
) -> Result<(), SCGError> {
    if !config.valid() {
        return Err(SCGError::InvalidConfig);
    }

    scg.LDOCSR().modify(|r| r.set_LDOEN(true));

    scg.SPLLCSR().modify(|r| r.set_LK(false));
    scg.SPLLCSR().modify(|r| {
        r.set_SPLLPWREN(false);
        r.set_SPLLCLKEN(false);
    });

    scg.SPLLCTRL().modify(|r| {
        r.set_SOURCE(config.source as u8);
        r.set_SELI(config.seli());
        r.set_SELP(config.selp());
        r.set_SELR(0);
    });

    scg.SPLLNDIV().write(|r| {
        r.set_NDIV(config.n);
        r.set_NREQ(true);
    });
    scg.SPLLPDIV().write(|r| {
        r.set_PDIV(config.p);
        r.set_PREQ(true);
    });
    scg.SPLLMDIV().write(|r| {
        r.set_MDIV(config.m);
        r.set_MREQ(true);
    });

    scg.SPLLSSCG0().write(|r| *r = 0);
    scg.SPLLSSCG1().write(|r| r.0 = 0);

    scg.TRIM_LOCK().write(|r| r.0 = 0x5A5A0001);

    let pre_div = if scg.SPLLCTRL().read().BYPASSPREDIV() {
        1
    } else if config.n == 0 {
        1
    } else {
        config.n as u32
    };

    let clk_rate = source_clk_freq / pre_div;
    scg.SPLLLOCK_CNFG()
        .write(|r| r.set_LOCK_TIME(clk_rate / 2000u32 + 300u32));

    scg.SPLLCSR().modify(|r| {
        r.set_SPLLPWREN(true);
        r.set_SPLLCLKEN(true);
    });
    scg.SPLLCSR().modify(|r| r.set_LK(true));

    while !scg.SPLLCSR().read().SPLL_LOCK() {}
    if scg.SPLLCSR().read().SPLLERR() {
        return Err(SCGError::SPLLErr);
    }

    Ok(())
}
