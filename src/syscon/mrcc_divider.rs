macro_rules! generate_mrcc_divider {
    ($name:ident, $reg_name:ident, $comment:expr) => {
        #[doc = $comment]
        pub fn $name(divider: Option<u8>) {
            let reg = unsafe { crate::pac::mrcc::MRCC0::instance().regs().$reg_name() };
            match divider {
                Some(divider) => {
                    reg.write(|r| {
                        r.set_DIV(divider);
                        r.set_HALT(false);
                        r.set_RESET(false);
                    });
                    while reg.read().UNSTAB() {}
                }
                None => reg.write(|r| {
                    r.set_DIV(0);
                    r.set_HALT(true);
                    r.set_RESET(false);
                }),
            }
        }
    };
}
macro_rules! generate_mrcc_clock_source {
    ($name:ident, $reg_name:ident, $st:ty, $comment:expr) => {
        #[doc = $comment]
        pub fn $name(clock: $st) {
            let reg = unsafe { crate::pac::mrcc::MRCC0::instance().regs().$reg_name() };
            reg.write(|r| r.set_MUX(clock as u8));
        }
    };
}

pub(crate) use generate_mrcc_clock_source;
pub(crate) use generate_mrcc_divider;

#[derive(Clone, Copy)]
pub enum ClkOutSource {
    FroLF = 0,
    FroHfDiv = 1,
    ClkIn = 2,
    Clk16K = 3,
    SPll = 5,
    SlowClk = 6,
    NoClock = 7,
}

generate_mrcc_clock_source!(
    setup_clkout_clock_source,
    MRCC_CLKOUT_CLKSEL,
    ClkOutSource,
    "Setup CLKOUT clock source."
);
generate_mrcc_divider!(
    setup_clkout_divider,
    MRCC_CLKOUT_CLKDIV,
    "Setup CLKOUT divider."
);
