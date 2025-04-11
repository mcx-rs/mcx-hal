#![no_std]
#![no_main]

use panic_halt as _;

#[rtic::app(device = mcx_pac, peripherals = false)]
mod app {
    use eio06::Write;
    use mcx_hal::prelude::*;
    use rtic_sync::{
        channel::{Receiver, Sender},
        make_channel,
    };

    /// LPUART2 with P2.2 and P2.3 as TX and RX
    type LpUart2 = LpUart<2, LpUartPins<PortPin<2, 2>, PortPin<2, 3>>>;

    #[shared]
    struct Shared {
        lpuart2: LpUart2,
    }

    #[local]
    struct Local {
        sender: Sender<'static, u8, 32>,
        receiver: Receiver<'static, u8, 32>,
    }

    #[init]
    fn init(_ctx: init::Context) -> (Shared, Local) {
        let mut scg = SCG::without_pins(unsafe { pac::scg::SCG0::instance() });
        let cfg = SCGConfig {
            firc_fclk_en: true,
            ..Default::default()
        };
        scg.freeze(&cfg).unwrap();
        setup_fro_hf_divider(Some(0));
        setup_lpuart2_clock_source(MRCCClockSource::FroHfDiv);
        setup_lpuart2_divider(Some(0));

        let port2 = Port2::new(unsafe { pac::port::PORT2::instance() });
        let mut lpuart2 = LpUart::new(
            unsafe { pac::lpuart::LPUART2::instance() },
            LpUartPins {
                tx: port2.p2,
                rx: port2.p3,
            },
        );
        lpuart2.configure(|i| {
            i.set_baud(&BaudRate::new(45_000_000, 115200).unwrap());
            i.set_tx_fifo(Some(0));
            i.set_rx_fifo(Some(0));
        });
        lpuart2.set_enable(LpUartDirection::TX, true);
        lpuart2.set_enable(LpUartDirection::RX, true);
        writeln!(lpuart2, "Hello Rust World!").unwrap();

        let (sender, receiver) = make_channel!(u8, 32);

        lpuart2.enable_interrupts(LpUartInterrupt::RECEIVER);
        unsafe { cortex_m::peripheral::NVIC::unmask(interrupt::LPUART2) }

        lpuart2_tx::spawn().unwrap();

        (Shared { lpuart2 }, Local { sender, receiver })
    }

    #[task(shared = [lpuart2], local= [receiver])]
    async fn lpuart2_tx(mut ctx: lpuart2_tx::Context) {
        while let Ok(b) = ctx.local.receiver.recv().await {
            ctx.shared.lpuart2.lock(|lpuart| {
                lpuart.write(&[b]).unwrap();
            });
        }
    }

    #[task(binds = LPUART2, shared = [lpuart2], local = [sender])]
    fn lpuart2_isr(ctx: lpuart2_isr::Context) {
        let mut lpuart2 = ctx.shared.lpuart2;
        let sender = ctx.local.sender;

        lpuart2.lock(|lpuart2| {
            if lpuart2.status().RDRF() {
                let b = unsafe { lpuart2.read_byte() };
                // let the buffer full, this is a demo.
                let _ = sender.try_send(b);
            }
        });
    }
}
