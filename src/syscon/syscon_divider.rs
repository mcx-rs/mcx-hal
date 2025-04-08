macro_rules! generate_syscon_divider {
    ($name: ident, $reg_name: ident, $comment:expr) => {
        #[doc = $comment]
        pub fn $name(divider: Option<u8>) {
            let reg = unsafe { crate::pac::syscon::SYSCON0::instance().regs().$reg_name() };
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

/// Setup AHBCLK divider.
/// This clock is divided from MAIN_CLK.
pub fn setup_ahbclk_divider(divider: u8) {
    let reg = unsafe { crate::pac::syscon::SYSCON0::instance().regs().AHBCLKDIV() };
    reg.write(|r| r.set_DIV(divider));
    while reg.read().UNSTAB() {}
}

generate_syscon_divider!(
    setup_fro_lf_divider,
    FROLFDIV,
    "Setup FRO_LF divider.\nThis clock is divided from SIRC_12M_CLK."
);
generate_syscon_divider!(
    setup_fro_hf_divider,
    FROHFDIV,
    "Setup FRO_HF divider.\nThis clock is divided from FIRC_FCLK."
);

#[cfg(feature = "mcxa2")]
generate_syscon_divider!(
    setup_spll_clk_divider,
    PLL1CLKDIV,
    "Setup SPLL_DIV divider.\nThis clock is divided from SPLL."
);
