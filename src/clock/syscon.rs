//! # SYSCON based clock control
//! MCX N Series uses SYSCON to control peripheral clocks.

use crate::pac;

pub trait ClockExt {
    fn enable(&self);

    fn disable(&self);
}

macro_rules! impl_clockext {
    ($([$name:tt, [$(($ahb:expr, $bit:expr),)*]]), +,) => {
        $(
            #[allow(unused_variables)]
            impl ClockExt for pac::$name {
                #[inline]
                fn enable(&self) {
                    let syscon = pac::SYSCON0::ptr();
                    $(
                        unsafe { (*syscon).ahbclkctrlset($ahb).write(|w| w.bits(1 << $bit)); }
                    )*
                }

                #[inline]
                fn disable(&self) {
                    let syscon = pac::SYSCON0::ptr();
                    $(
                        unsafe { (*syscon).ahbclkctrlclr($ahb).write(|w| w.bits(1 << $bit)); }
                    )*
                }
            }
        )+

        #[allow(non_snake_case)]
        pub mod PeripheralClocks {
            $(
                #[allow(non_snake_case, unused_variables)]
                pub mod $name {
                    pub fn enable() {
                        let syscon = $crate::pac::SYSCON0::ptr();
                        $(
                            unsafe { (*syscon).ahbclkctrlset($ahb).write(|w| w.bits(1 << $bit)); }
                        )*
                    }

                    pub fn disable() {
                        let syscon = $crate::pac::SYSCON0::ptr();
                        $(
                            unsafe { (*syscon).ahbclkctrlclr($ahb).write(|w| w.bits(1 << $bit)); }
                        )*
                    }
                }
            )+
        }
    };
}

impl_clockext!(
    // [ROM, 0, 1],
    // [SRAM1, 0, 2],
    // [SRAM2, 0, 3],
    // [SRAM3, 0, 4],
    // [SRAM4, 0, 5],
    // [SRAM5, 0, 6],
    // [SRAM6, 0, 7],
    // [SRAM7, 0, 8],
    [FMU0, [(0, 9),]],
    [NPX0, [(0, 10),]],
    [FLEXSPI0, [(0, 11),]],
    [INPUTMUX0, [(0, 12),]],
    [PORT0, [(0, 13),]],
    [PORT1, [(0, 14),]],
    [PORT2, [(0, 15),]],
    [PORT3, [(0, 16),]],
    [PORT4, [(0, 17),]],
    [PORT5, []],
    // // missing 0-18
    [GPIO0, [(0, 19),]],
    [GPIO1, [(0, 20),]],
    [GPIO2, [(0, 21),]],
    [GPIO3, [(0, 22),]],
    [GPIO4, [(0, 23),]],
    [GPIO5, []],
    // // missing 0-24
    // [PINT0, 0, 25],
    // [DMA0, 0, 26],
    // [CRC0, 0, 27],
    // [WWDT0, 0, 28],
    // [WWDT1, 0, 29],
    // // missing 0-30
    // [MAILBOX, 0, 31],
    // [MRT0, 1, 0],
    // [OSTIMER0, 1, 1],
    // [SCT0, 1, 2],
    // [ADC0, 1, 3],
    // [ADC1, 1, 4],
    // [DAC0, 1, 5],
    // [RTC0, 1, 6],
    // // missing 1-7
    // [EMVSIM0, 1, 8],
    // [EMVSIM1, 1, 9],
    // [UTICK0, 1, 10],
    // [LP_FLEXCOMM0, 1, 11],
    // [LP_FLEXCOMM1, 1, 12],
    // [LP_FLEXCOMM2, 1, 13],
    // [LP_FLEXCOMM3, 1, 14],
    // [LP_FLEXCOMM4, 1, 15],
    // [LP_FLEXCOMM5, 1, 16],
    // [LP_FLEXCOMM6, 1, 17],
    // [LP_FLEXCOMM7, 1, 18],
    // [LP_FLEXCOMM8, 1, 19],
    // [LP_FLEXCOMM9, 1, 20],
    // [LPUART0, 1, 11],
    // [LPUART1, 1, 12],
    // [LPUART2, 1, 13],
    // [LPUART3, 1, 14],
    // [LPUART4, 1, 15],
    // [LPUART5, 1, 16],
    // [LPUART6, 1, 17],
    // [LPUART7, 1, 18],
    // [LPUART8, 1, 19],
    // [LPUART9, 1, 20],
    // [LPSPI0, 1, 11],
    // [LPSPI1, 1, 12],
    // [LPSPI2, 1, 13],
    // [LPSPI3, 1, 14],
    // [LPSPI4, 1, 15],
    // [LPSPI5, 1, 16],
    // [LPSPI6, 1, 17],
    // [LPSPI7, 1, 18],
    // [LPSPI8, 1, 19],
    // [LPSPI9, 1, 20],
    // [PDM, 1, 21],
    // [CTIMER2, 1, 22],
    // // [USB0RAM, 1, 23],
    // [USBDCD0, 1, 24],
    // [USBFS0, 1, 25],
    // [CTIMER0, 1, 26],
    // [CTIMER1, 1, 27],
    // // missing 1-28
    // // [PKCRAM, 1, 29],
    // // missing 1-30
    // [SMARTDMA0, 1, 31],
    // // [ESPI, 2, 0],
    // [DMA1, 2, 1],
    // [ENET0, 2, 2],
    // [USDHC0, 2, 3],
    // [FLEXIO0, 2, 4],
    // [SAI0, 2, 5],
    // [SAI1, 2, 6],
    // // [TRO, 2, 7],
    // [FREQME0, 2, 8],
    // // missing 2-9 to 2-12
    // [TRNG0, 2, 13],
    // [CAN0, 2, 14],
    // [CAN1, 2, 15],
    // [USBHS1__USBC, 2, 16],
    // [USBHS1__USBNC, 2, 16],
    // [USBHS1_PHY_DCD, 2, 17],
    // // [CSS, 2, 18],
    // [POWERQUAD, 2, 19],
    // [PLU0, 2, 20],
    // [CTIMER3, 2, 21],
    // [CTIMER4, 2, 22],
    // [PUF, 2, 23],
    // [PKC0, 2, 24],
    // // missing 2-25
    [SCG0, [(2, 26),]],
    // // missing 2-27, 2-28
    // [GDET0, 2, 29],
    // [SM3_0, 2, 30],
    // [I3C0, 3, 0],
    // [I3C1, 3, 1],
    // [SINC0, 3, 2],
    // [BSP32_0, 3, 3],
    // [ENC0, 3, 4],
    // [ENC1, 3, 5],
    // [PWM0, 3, 6],
    // [PWM1, 3, 7],
    // [EVTG0, 3, 8],
    // // missing 3-9, 3-10
    // [DAC1, 3, 11],
    // [DAC2, 3, 12],
    // [OPAMP0, 3, 13],
    // [OPAMP1, 3, 14],
    // [OPAMP2, 3, 15],
    // // missing 3-16, 3-17
    // [CMP2, 3, 18],
    // [VREF0, 3, 19],
    // [BSP32_0, 3, 20],
    // // [Neutron, 3, 21],
    // // [TSI, 3, 22],
    // // [EWM, 3, 23],
    // // [EIM, 3, 24],
    // // [ERM, 3, 25],
    // // [INTM, 3, 26],
    // // [SEMA42, 3, 27],
);