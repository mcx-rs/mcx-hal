#![no_std]
#![no_main]

extern crate panic_halt;

use eio06::Write;
use mcx_hal::prelude::*;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut scg = SCG::without_pins(unsafe { pac::scg::SCG0::instance() });
    let cfg = SCGConfig {
        firc_fclk_en: true,
        ..Default::default()
    };
    scg.freeze(&cfg).unwrap();

    let port2 = Port2::new(unsafe { pac::port::PORT2::instance() });

    setup_fro_hf_divider(Some(0));
    setup_lpuart2_clock_source(mcx_hal::syscon::MRCCClockSource::FroHfDiv);
    setup_lpuart2_divider(Some(0));

    let mut lpuart2 = LpUart::new(
        unsafe { pac::lpuart::LPUART2::instance() },
        LpUartPins {
            tx: port2.p2,
            rx: port2.p3,
        },
    );
    lpuart2.configure(|i| {
        i.set_baud(&BaudRate::new(FIRC::default().freq(), 115200).unwrap());
        i.set_rx_fifo(Some(0));
    });
    lpuart2.set_enable(LpUartDirection::TX, true);
    lpuart2.set_enable(LpUartDirection::RX, true);

    writeln!(
        lpuart2,
        "Hello World!, current FIRC {:?}, {}",
        cfg.firc, cfg.firc_fclk_en
    )
    .unwrap();

    loop {}
}
