#![no_std]
#![no_main]

use panic_halt as _;

#[rtic::app(device = mcx_pac, peripherals = false)]
mod app {
    use mcx_hal::{gpio::Input, port::Port, prelude::*};
    use rtic_monotonics::{rtic_time::embedded_hal_async::delay::DelayNs, systick_monotonic};

    systick_monotonic!(Mono);

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        btn: Input<PortPin<1, 7>>,

        led_r: Output<PortPin<3, 18>>,
        led_g: Output<PortPin<3, 19>>,
        led_b: Output<PortPin<3, 21>>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        Mono::start(ctx.core.SYST, 45_000_000);

        let port1 = Port1::new(unsafe { pac::port::PORT1::instance() });
        let port3: Port3 = Port3::new(unsafe { pac::port::PORT3::instance() });
        let mut gpio1 = GPIO::new(unsafe { pac::gpio::GPIO1::instance() });
        let mut gpio3 = GPIO::new(unsafe { pac::gpio::GPIO3::instance() });

        let led_r = gpio3.output(port3.p18);
        let led_g = gpio3.output(port3.p19);
        let led_b = gpio3.output(port3.p21);

        led_r.set();
        led_g.set();
        led_b.set();

        let mut btn = gpio1.input(port1.p7);
        btn.mut_pin().floating();
        btn.mut_pin().analog(false);
        btn.set_interrupt_config(GPIOIRQConfig::InterruptFallingEdge);
        unsafe { cortex_m::peripheral::NVIC::unmask(interrupt::GPIO1) }

        blink::spawn().unwrap();

        (
            Shared {},
            Local {
                btn,
                led_r,
                led_g,
                led_b,
            },
        )
    }

    #[task(binds = GPIO1, local = [btn, led_b])]
    fn btn(ctx: btn::Context) {
        let btn = ctx.local.btn;
        let led_b = ctx.local.led_b;

        btn.clear_interrupt_flag();
        led_b.toggle().unwrap();
    }

    #[task(local = [led_r, led_g])]
    async fn blink(ctx: blink::Context) {
        let led_r = ctx.local.led_r;
        let led_g = ctx.local.led_g;

        loop {
            Mono.delay_ms(500).await;
            led_r.toggle().unwrap();
            led_g.toggle().unwrap();
        }
    }
}
