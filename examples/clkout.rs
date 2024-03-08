#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use mcx_hal::{self as hal, pac};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut clocks = hal::clock::Clocks::constrain(dp.SCG0);
    let gpio2 = hal::gpio::gpio2::split(dp.GPIO2, dp.PORT2);

    const CLOCK_CONFIG: hal::clock::Config = hal::clock::Config {
        sosc_mode: None,
        firc_range: Some(hal::clock::FIRCRange::FIRC144M),
        main_clock_source: hal::clock::MainClockSource::FIRC,
        firc_fclk_periph_en: true,
        firc_sclk_periph_en: true,
        sirc_clk_periph_en: true,
        sirc_div12_en: true,
        ahbclkdiv: 0,
    };

    clocks.use_config(CLOCK_CONFIG);
    clocks.freeze();
    let _clkout = gpio2.pio2_2.into_push_pull_output().into_mux::<1>();

    hal::clock::clkout::setup(hal::clock::clkout::ClockSource::MainClock, 11 - 1);
    hal::clock::clkout::run();

    loop {}
}
