use crate::pac::*;
use crate::syscon::{generate_mrcc_divider, periph_en_define};

use super::generate_mrcc_clock_source;

periph_en_define! {
    (periph: inputmux::INPUTMUX0, 0,  0, hRST: true, hCC: true, hACC: true)
    (periph: i3c::I3C0,           0,  1, hRST: true, hCC: true, hACC: true)
    (periph: ctimer::CTIMER0,     0,  2, hRST: true, hCC: true, hACC: true)
    (periph: ctimer::CTIMER1,     0,  3, hRST: true, hCC: true, hACC: true)
    (periph: ctimer::CTIMER2,     0,  4, hRST: true, hCC: true, hACC: true)
    (periph: ctimer::CTIMER3,     0,  5, hRST: true, hCC: true, hACC: true)
    (periph: ctimer::CTIMER4,     0,  6, hRST: true, hCC: true, hACC: true)
    (periph: freqme::FREQME0,     0,  7, hRST: true, hCC: true, hACC: true)
    (periph: utick::UTICK0,       0,  8, hRST: true, hCC: true, hACC: true)
    (periph: wwdt::WWDT0,         0,  9,             hCC: true, hACC: true)
    (periph: smartdma::SMARTDMA0, 0, 10, hRST: true, hCC: true, hACC: true)
    (periph: dma::DMA0,           0, 11, hRST: true, hCC: true, hACC: true)
    (periph: aoi::AOI0,           0, 12, hRST: true, hCC: true, hACC: true)
    (periph: crc::CRC0,           0, 13, hRST: true, hCC: true, hACC: true)
    (periph: eim::EIM0,           0, 14, hRST: true, hCC: true, hACC: true)
    (periph: erm::ERM0,           0, 15, hRST: true, hCC: true, hACC: true)
    (periph: fmc::FMC0,           0, 16,             hCC: true, hACC: true)
    (periph: aoi::AOI1,           0, 17, hRST: true, hCC: true, hACC: true)
    (periph: flexio::FLEXIO0,     0, 18, hRST: true, hCC: true, hACC: true)
    (periph: lpi2c::LPI2C0,       0, 19, hRST: true, hCC: true, hACC: true)
    (periph: lpi2c::LPI2C1,       0, 20, hRST: true, hCC: true, hACC: true)
    (periph: lpspi::LPSPI0,       0, 21, hRST: true, hCC: true, hACC: true)
    (periph: lpspi::LPSPI1,       0, 22, hRST: true, hCC: true, hACC: true)
    (periph: lpuart::LPUART0,     0, 23, hRST: true, hCC: true, hACC: true)
    (periph: lpuart::LPUART1,     0, 24, hRST: true, hCC: true, hACC: true)
    (periph: lpuart::LPUART2,     0, 25, hRST: true, hCC: true, hACC: true)
    (periph: lpuart::LPUART3,     0, 26, hRST: true, hCC: true, hACC: true)
    (periph: lpuart::LPUART4,     0, 27, hRST: true, hCC: true, hACC: true)
    (periph: usb::USB0,           0, 28, hRST: true, hCC: true, hACC: true)
    (periph: eqdc::EQDC0,         0, 29, hRST: true, hCC: true, hACC: true)
    (periph: eqdc::EQDC1,         0, 30, hRST: true, hCC: true, hACC: true)
    (periph: pwm::FLEXPWM0,       0, 31, hRST: true, hCC: true, hACC: true)

    (periph: pwm::FLEXPWM1,       1,  0, hRST: true, hCC: true, hACC: true)
    (periph: ostimer::OSTIMER0,   1,  1, hRST: true, hCC: true, hACC: true)
    (periph: adc::ADC0,           1,  2, hRST: true, hCC: true, hACC: true)
    (periph: adc::ADC1,           1,  3, hRST: true, hCC: true, hACC: true)
    (periph: lpcmp::CMP0,         1,  4,             hCC: true, hACC: true)
    (periph: lpcmp::CMP1,         1,  5, hRST: true, hCC: true, hACC: true)
    (periph: lpcmp::CMP2,         1,  6, hRST: true, hCC: true, hACC: true)
    (periph: lpdac::DAC0,         1,  7, hRST: true, hCC: true, hACC: true)
    (periph: opamp::OPAMP0,       1,  8, hRST: true, hCC: true, hACC: true)
    (periph: opamp::OPAMP1,       1,  9, hRST: true, hCC: true, hACC: true)
    (periph: opamp::OPAMP2,       1, 10, hRST: true, hCC: true, hACC: true)
    (periph: opamp::OPAMP3,       1, 11, hRST: true, hCC: true, hACC: true)
    (periph: port::PORT0,         1, 12, hRST: true, hCC: true, hACC: true)
    (periph: port::PORT1,         1, 13, hRST: true, hCC: true, hACC: true)
    (periph: port::PORT2,         1, 14, hRST: true, hCC: true, hACC: true)
    (periph: port::PORT3,         1, 15, hRST: true, hCC: true, hACC: true)
    (periph: port::PORT4,         1, 16, hRST: true, hCC: true, hACC: true)
    (periph: slcd::SLCD0,         1, 17, hRST: true, hCC: true, hACC: true)
    (periph: can::CAN0,           1, 18, hRST: true, hCC: true, hACC: true)
    (periph: can::CAN1,           1, 19, hRST: true, hCC: true, hACC: true)
    (periph: lpi2c::LPI2C2,       1, 20, hRST: true, hCC: true, hACC: true)
    (periph: lpi2c::LPI2C3,       1, 21, hRST: true, hCC: true, hACC: true)
    (periph: lpuart::LPUART5,     1, 22, hRST: true, hCC: true, hACC: true)
    (periph: tdet::TDET0,         1, 23,             hCC: true)
    (periph: pkc::PKC0,           1, 24, hRST: true, hCC: true, hACC: true)
    (periph: sgi::SGI0,           1, 25,             hCC: true, hACC: true)
    (periph: trng::TRNG0,         1, 26, hRST: true, hCC: true, hACC: true)
    (periph: udf::UDF0,           1, 27,             hCC: true, hACC: true)
    (periph: adc::ADC2,           1, 28, hRST: true, hCC: true, hACC: true)
    (periph: adc::ADC3,           1, 29, hRST: true, hCC: true, hACC: true)

    (virt: RAMA,                  2,  1,             hCC: true, hACC: true)
    (virt: RAMB,                  2,  2,             hCC: true, hACC: true)
    (virt: RAMC,                  2,  3,             hCC: true, hACC: true)
    (periph: gpio::GPIO0,         2,  4, hRST: true, hCC: true, hACC: true)
    (periph: gpio::GPIO1,         2,  5, hRST: true, hCC: true, hACC: true)
    (periph: gpio::GPIO2,         2,  6, hRST: true, hCC: true, hACC: true)
    (periph: gpio::GPIO3,         2,  7, hRST: true, hCC: true, hACC: true)
    (periph: gpio::GPIO4,         2,  8, hRST: true, hCC: true, hACC: true)
    (periph: mau::MAU0,           2,  9,             hCC: true, hACC: true)
    (virt: ROMC,                  2, 10,             hCC: true, hACC: true)
}

/// Clock Source definition used in MRCC clock divider.
#[repr(u8)]
pub enum MRCCClockSource {
    FroLfDiv = 0,
    FroHf = 1,
    FroHfDiv = 2,
    ClkIn = 3,
    Clk16K = 4,
    Clk1M = 5,
    SPllDiv = 6,
    NoClock = 7,
}

macro_rules! generate_lpuart_clock_source_and_divider {
    ($($instance:literal),+) => {
        paste::paste! {
            $(
                generate_mrcc_divider!( [< setup_lpuart $instance _divider >], [< MRCC_LPUART $instance _CLKDIV >], concat!("Setup LPUART", $instance, " divider"));
                generate_mrcc_clock_source!( [< setup_lpuart $instance _clock_source >], [< MRCC_LPUART $instance _CLKSEL >], MRCCClockSource, concat!("Setup LPUART", $instance, " clock source"));
            )+
        }
    };
}
generate_lpuart_clock_source_and_divider!(0, 1, 2, 3, 4, 5);

macro_rules! generate_ctimer_clock_source_and_divider {
    ($($instance:literal),+) => {
        paste::paste! {
            $(
                generate_mrcc_divider!( [< setup_ctimer $instance _divider >], [< MRCC_CTIMER $instance _CLKDIV >], concat!("Setup CTIMER", $instance, " divider"));
                generate_mrcc_clock_source!( [< setup_ctimer $instance _clock_source >], [< MRCC_CTIMER $instance _CLKSEL >], MRCCClockSource, concat!("Setup CTIMER", $instance, " clock source"));
            )+
        }
    };
}
generate_ctimer_clock_source_and_divider!(0, 1, 2, 3, 4);
