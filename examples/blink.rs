#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use embedded_hal::digital::StatefulOutputPin;
use mcx_hal::{self as hal, pac};

#[entry]
fn main() -> ! {
    let cp = pac::CorePeripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let gpio0 = hal::gpio::gpio0::split(dp.GPIO0, dp.PORT0);
    let mut led_r = gpio0.pio0_10.into_push_pull_output();

    let mut clocks = hal::clock::Clocks::constrain(dp.SCG0);
    const CLOCK_CONFIG: hal::clock::Config = hal::clock::Config::new_frohf48m();
    clocks.use_config(CLOCK_CONFIG);
    clocks.freeze();

    const SYSTICK_DIV: u8 = 2;
    hal::clock::systick::setup_systick(
        0,
        hal::clock::systick::ClockSource::MainClock(SYSTICK_DIV - 1),
    );

    let mut delay = cortex_m::delay::Delay::new(
        cp.SYST,
        CLOCK_CONFIG.get_main_clk().unwrap() / SYSTICK_DIV as u32,
    );

    loop {
        led_r.toggle().unwrap();
        delay.delay_ms(2_000u32);
    }
}
