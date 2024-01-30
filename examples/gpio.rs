#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;

use mcx_hal::{self as hal, pac};

#[entry]
fn main() -> ! {
    let cp = pac::CorePeripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let port0 = hal::gpio::port0::Parts::new(dp.PORT0, dp.GPIO0);

    loop {}
}
