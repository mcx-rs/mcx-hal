#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;

use mcx_hal::{self as hal, pac};

#[entry]
fn main() -> ! {
    let cp = pac::CorePeripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let gpio0 = hal::gpio::gpio0::split(dp.GPIO0, dp.PORT0);

    let mut pio0_0 = gpio0.pio0_0;
    let mut pio0_0 = pio0_0.into_mux::<10>();

    loop {}
}
