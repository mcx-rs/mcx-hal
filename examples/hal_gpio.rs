#![no_std]
#![no_main]

use core::{
    cell::RefCell,
    sync::atomic::{AtomicBool, Ordering},
};

use cortex_m::interrupt::Mutex;
use mcx_hal::prelude::*;
use panic_halt as _;

type Btn = Input<PortPin<1, 7>>;

static FLAG_BTN_PRESSED: AtomicBool = AtomicBool::new(false);
static BTN: Mutex<RefCell<Option<Btn>>> = Mutex::new(RefCell::new(None));

#[cortex_m_rt::entry]
fn main() -> ! {
    let port1 = Port1::new(unsafe { pac::port::PORT1::instance() });
    let port3 = Port3::new(unsafe { pac::port::PORT3::instance() });
    let mut gpio1 = GPIO::new(unsafe { pac::gpio::GPIO1::instance() });
    let mut gpio3 = GPIO::new(unsafe { pac::gpio::GPIO3::instance() });

    let led = gpio3.output(port3.p18);
    let mut btn = gpio1.input(port1.p7);

    btn.mut_pin().floating();
    btn.mut_pin().analog(false);
    btn.set_interrupt_config(GPIOIRQConfig::InterruptFallingEdge);
    unsafe { cortex_m::peripheral::NVIC::unmask(interrupt::GPIO1) }
    unsafe { cortex_m::interrupt::enable() }

    cortex_m::interrupt::free(|cs| BTN.borrow(cs).borrow_mut().replace(btn));

    loop {
        if FLAG_BTN_PRESSED.load(Ordering::Relaxed) {
            led.toggle();
            FLAG_BTN_PRESSED.store(false, Ordering::Relaxed);
        }
    }
}

#[interrupt]
unsafe fn GPIO1() {
    // pac::gpio::GPIO1::instance()
    //     .regs()
    //     .ICR(7)
    //     .modify(|r| r.set_ISF(true));

    cortex_m::interrupt::free(|cs| {
        BTN.borrow(cs)
            .borrow_mut()
            .as_mut()
            .unwrap()
            .clear_interrupt_flag()
    });

    FLAG_BTN_PRESSED.store(true, Ordering::Relaxed);
}
