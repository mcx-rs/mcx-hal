#![no_std]
#![no_main]

use embedded_hal::digital::StatefulOutputPin;
use panic_halt as _;

use core::cell::{Cell, RefCell};
use cortex_m::asm::wfi;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use mcx_hal::{self as hal, pac, pac::interrupt};

type BtnType = hal::gpio::gpio0::PIO0_6<hal::gpio::Input<hal::gpio::Floating>>;

static FLAG_BTN_PRESSED: Mutex<Cell<bool>> = Mutex::new(Cell::new(false));
static BTN: Mutex<RefCell<Option<BtnType>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let gpio0 = hal::gpio::gpio0::split(dp.GPIO0, dp.PORT0);
    let mut btn = gpio0.pio0_6.into_floating_input();
    let mut led_r = gpio0.pio0_10.into_push_pull_output();

    btn.enable_irq(
        hal::gpio::GPIOInterruptSource::FallingEdge,
        hal::gpio::GPIOInterruptSelect::IRQ0,
    );
    cortex_m::interrupt::free(|cs| {
        BTN.borrow(cs).replace(Some(btn));
    });

    // enable GPIO0 irq
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::GPIO00);
    }

    loop {
        wfi();

        cortex_m::interrupt::free(|cs| {
            if FLAG_BTN_PRESSED.borrow(cs).get() {
                FLAG_BTN_PRESSED.borrow(cs).set(false);
                led_r.toggle().unwrap();
            }
        });
    }
}

#[interrupt]
fn GPIO00() {
    cortex_m::interrupt::free(|cs| {
        let mut btn = BTN.borrow(cs).borrow_mut();
        btn.as_mut().unwrap().clear_irq_flag();
        FLAG_BTN_PRESSED.borrow(cs).set(true);
    });
}
