#![no_std]
#![no_main]

extern crate panic_halt;

use eio06::Write;
use mcx_hal::{
    lpuart::{BaudRate, LpUart, Pins},
    pac,
    port::Port2,
    scg::{Config as ClockConfig, FIRC, SCG},
    syscon::{setup_fro_hf_divider, setup_lpuart2_clock_source, setup_lpuart2_divider},
};

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut scg = SCG::without_pins(unsafe { pac::scg::SCG0::instance() });

    let mut cfg = ClockConfig::default();
    cfg.firc_fclk_en = true;
    scg.freeze(&cfg).unwrap();

    let port2 = Port2::new(unsafe { pac::port::PORT2::instance() });

    setup_fro_hf_divider(Some(0));
    setup_lpuart2_clock_source(mcx_hal::syscon::MRCCClockSource::FroHfDiv);
    setup_lpuart2_divider(Some(0));

    let mut lpuart2 = LpUart::new(
        unsafe { pac::lpuart::LPUART2::instance() },
        Pins {
            tx: port2.p2,
            rx: port2.p3,
        },
    );
    lpuart2.configure(|i| {
        i.set_baud(&BaudRate::new(FIRC::default().freq(), 115200).unwrap());
    });
    lpuart2.set_enable(mcx_hal::lpuart::Direction::TX, true);

    writeln!(
        lpuart2,
        "Hello World!, current FIRC {:?}, {}",
        cfg.firc, cfg.firc_fclk_en
    )
    .unwrap();

    loop {}
}
