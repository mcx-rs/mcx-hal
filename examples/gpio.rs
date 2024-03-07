#![no_std]
#![no_main]

use embedded_hal::digital::StatefulOutputPin;
use panic_halt as _;

use core::cell::Cell;
use cortex_m::asm::wfi;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use mcx_hal::{self as hal, pac, pac::interrupt};

static FLAG_BTN_PRESSED: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));

#[entry]
fn main() -> ! {
    let cp = pac::CorePeripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let gpio0 = hal::gpio::gpio0::split(dp.GPIO0, dp.PORT0);
    let mut btn = gpio0.pio0_6.into_floating_input();
    let mut led_r = gpio0.pio0_10.into_push_pull_output();

    btn.enable_irq(
        hal::gpio::GPIOInterruptSource::FallingEdge,
        hal::gpio::GPIOInterruptSelect::IRQ0,
    );

    // enable GPIO0 irq
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::GPIO00);
    }

    loop {
        wfi();

        cortex_m::interrupt::free(|cs| {
            if FLAG_BTN_PRESSED.borrow(cs).get() {
                led_r.toggle();
                FLAG_BTN_PRESSED.borrow(cs).set(false);
            }
        });
    }
}

#[interrupt]
fn GPIO00() {
    cortex_m::interrupt::free(|cs| FLAG_BTN_PRESSED.borrow(cs).set(true));
}
