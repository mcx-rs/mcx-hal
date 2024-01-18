//! SCG(System Clock Generator) Configurations
use crate::clock::Error;

macro_rules! scg_register {
    (@get_ptr) => {
        $crate::pac::SCG0::ptr()
    };

    (@write $register:ident, $lambda:expr) => {
        unsafe {
            (*scg_register!(@get_ptr)).$register().write($lambda);
        }
    };

    (@read $register:ident) => {
       unsafe { (*scg_register!(@get_ptr)).$register().read() }
    };

    (@modify $register:ident, $lambda:expr) => {
        unsafe {
            (*scg_register!(@get_ptr)).$register().modify($lambda);
        }
    };

    (@reset $register:ident) => {
        unsafe {
            (*scg_register!(@get_ptr)).$register().reset();
        }
    };
}

/// SOSC(System Crystal Oscillator Clock) Mode
#[derive(Debug, Clone, Copy)]
pub enum SOSCMode {
    Disabled,
    ExternalClock,
    Oscillator,
}

/// SOSC(System Crystal Oscillator Clock)
#[derive(Debug, Clone, Copy)]
pub struct SOSC {
    pub mode: SOSCMode,
    pub freq: u32,
}

impl Default for SOSC {
    fn default() -> Self {
        Self {
            mode: SOSCMode::Disabled,
            freq: 0,
        }
    }
}

impl SOSC {
    pub fn new(mode: SOSCMode, freq: u32) -> Self {
        Self { mode, freq }
    }

    pub fn apply(&self) -> Result<(), Error> {
        if match self.mode {
            SOSCMode::Disabled => true,
            _ => false,
        } {
            return Self::disable();
        }

        if Self::is_busy() {
            return Err(Error::Busy);
        }
        unsafe {
            Self::unlock();
        }
        let range = Self::range(self.freq)?;

        scg_register!(@modify sosccsr, |_, w| w.soscerr().enabled_and_error());
        scg_register!(@modify ldocsr, |_, w| w.ldoen().enabled());
        scg_register!(@modify sosccfg, |_, w| {
            match self.mode {
                SOSCMode::Disabled => unreachable!(),
                SOSCMode::ExternalClock => w.erefs().external(),
                SOSCMode::Oscillator => w.erefs().internal(),
            };
            w.range().bits(range)
        });

        scg_register!(@modify sosccsr, |_, w| { w.sosccm().enabled(); w.soscen().enabled() });

        Self::wait_valid();
        if Self::has_error() {
            return Err(Error::Unknown);
        }
        Self::lock();

        Ok(())
    }

    pub fn disable() -> Result<(), Error> {
        if Self::is_busy() {
            return Err(Error::Busy);
        }

        scg_register!(@reset sosccsr);
        scg_register!(@reset sosccfg);

        Self::lock();

        Ok(())
    }

    /// lock SOSC configurations and prevent any changes
    pub fn lock() {
        scg_register!(@modify sosccsr, |_, w| {w.lk().write_disabled()});
    }

    /// unlock SOSC configurations
    pub unsafe fn unlock() {
        scg_register!(@modify sosccsr, |_, w| w.lk().write_enabled())
    }

    fn range(freq: u32) -> Result<u8, Error> {
        match freq {
            16_000_000u32..=19_999_999u32 => Ok(0),
            20_000_000u32..=29_999_999u32 => Ok(1),
            30_000_000u32..=49_999_999u32 => Ok(2),
            50_000_000u32..=65_999_999u32 => Ok(3),
            _ => {
                return Err(Error::InvalidFrequency);
            }
        }
    }

    #[inline]
    fn is_busy() -> bool {
        if scg_register!(@read sosccsr).soscsel().is_sosc()
            || (scg_register!(@read apllctrl).source().is_sosc()
                && scg_register!(@read apllcsr).apllsel().is_apll())
            || (scg_register!(@read spllctrl).source().is_sosc()
                && scg_register!(@read spllcsr).spllsel().is_spll())
        {
            false
        } else {
            true
        }
    }

    #[inline]
    fn wait_valid() {
        while scg_register!(@read sosccsr).soscvld().is_disabled() {}
    }

    #[inline]
    fn has_error() -> bool {
        scg_register!(@read sosccsr)
            .soscerr()
            .is_enabled_and_error()
    }

    pub fn apply_external_clock<const FREQ: u32>() -> Result<(), Error> {
        struct CHECK<const F: u32>;
        impl<const F: u32> CHECK<F> {
            const RANGE: u8 = match F {
                16_000_000u32..=19_999_999u32 => 0,
                20_000_000u32..=29_999_999u32 => 1,
                30_000_000u32..=49_999_999u32 => 2,
                50_000_000u32..=65_999_999u32 => 3,
                _ => panic!("invalid frequency"),
            };
        }

        if Self::is_busy() {
            return Err(Error::Busy);
        }

        scg_register!(@write sosccsr, |w| w.soscerr().enabled_and_error());
        scg_register!(@modify ldocsr, |_, w| w.ldoen().enabled());
        scg_register!(@modify sosccfg, |_, w| { w.erefs().external(); w.range().bits(CHECK::<FREQ>::RANGE) });
        scg_register!(@modify sosccsr, |_, w| { w.sosccm().enabled(); w.soscen().enabled() });

        Self::wait_valid();
        Self::lock();

        Ok(())
    }

    pub fn apply_oscillator<const FREQ: u32>() -> Result<(), Error> {
        struct CHECK<const F: u32>;
        impl<const F: u32> CHECK<F> {
            const RANGE: u8 = match F {
                16_000_000u32..=19_999_999u32 => 0,
                20_000_000u32..=29_999_999u32 => 1,
                30_000_000u32..=49_999_999u32 => 2,
                50_000_000u32..=65_999_999u32 => 3,
                _ => panic!("invalid frequency"),
            };
        }

        if Self::is_busy() {
            return Err(Error::Busy);
        }

        scg_register!(@write sosccsr, |w| w.soscerr().enabled_and_error());
        scg_register!(@modify ldocsr, |_, w| w.ldoen().enabled());
        scg_register!(@write sosccfg, |w| { w.erefs().internal(); w.range().bits(CHECK::<FREQ>::RANGE) });
        scg_register!(@modify sosccsr, |_, w| { w.sosccm().enabled(); w.soscen().enabled() });

        Self::wait_valid();
        if Self::has_error() {
            return Err(Error::Unknown);
        }
        Self::lock();

        Ok(())
    }
}
