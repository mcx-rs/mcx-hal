#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use mcx_hal::{self as hal, pac};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut clocks = hal::clock::Clocks::constrain(dp.SCG0);

    let gpio1 = hal::gpio::gpio1::split(dp.GPIO1, dp.PORT1);
    let pio1_8 = gpio1.pio1_8.into_mux::<2>();
    let pio1_9 = gpio1.pio1_9.into_mux::<2>();

    let lpuart4 = hal::lpuart::lpuart4::new(dp.LPUART4, pio1_9, pio1_8).unwrap();

    loop {}
}
