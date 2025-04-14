#![no_std]
#![no_main]

use panic_halt as _;

use mcx_hal::prelude::*;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut scg = SCG::without_pins(unsafe { pac::scg::SCG0::instance() });
    let cfg = SCGConfig {
        firc_sclk_en: true,
        spll: PllConfig {
            source: PllSource::FIRC,
            n: Some(1),
            m: 8,
            p: Some(2),
            p2: false,
        }
        .into(),
        ..Default::default()
    };
    scg.freeze(&cfg).unwrap();

    let port4 = Port4::new(unsafe { pac::port::PORT4::instance() });
    let mut clkout = port4.p2;
    clkout.floating();
    clkout.analog(false);
    clkout.set_mux(1);

    setup_clkout_clock_source(ClkOutSource::SPll);
    setup_clkout_divider(Some(8));

    loop {}
}
