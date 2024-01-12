use crate::pac;

macro_rules! define_peripherals {
    ( $( [$name:tt, $ahb:expr, $bit:expr] ), +, ) => {
        // define_peripherals!(@define_enum $($name,)+);
        #[derive(Debug, Clone, Copy)]
        pub enum Peripherals {
            $( $name, )+
        }

        impl Peripherals {
            pub fn enable(&self) {
                let syscon = pac::SYSCON0::ptr();
                match self {
                    // $(define_peripherals!(@define_match_arm $name, $ahb, $bit))+
                    $(
                        Peripherals::$name => unsafe {
                            (*syscon).ahbclkctrlset($ahb).write(|w| w.bits(1 << $bit));
                        },
                    )+
                }
            }

            pub fn disable(&self) {
                let syscon = pac::SYSCON0::ptr();
                match self {
                    // $(define_peripherals!(@define_match_arm $name, $ahb, $bit))+
                    $(
                        Peripherals::$name => unsafe {
                            (*syscon).ahbclkctrlclr($ahb).write(|w| w.bits(1 << $bit));
                        },
                    )+
                }
            }
        }
    };
}

define_peripherals!(
    [ROM, 0, 1],
    [SRAM1, 0, 2],
    [SRAM2, 0, 3],
    [SRAM3, 0, 4],
    [SRAM4, 0, 5],
    [SRAM5, 0, 6],
    [SRAM6, 0, 7],
    [SRAM7, 0, 8],
    [FMU, 0, 9],
    [FMC, 0, 10],
    [FlexSPI, 0, 11],
    [InputMux, 0, 12],
    [Port0, 0, 13],
    [Port1, 0, 14],
    [Port2, 0, 15],
    [Port3, 0, 16],
    [Port4, 0, 17],
    // missing 0-18
    [GPIO0, 0, 19],
    [GPIO1, 0, 20],
    [GPIO2, 0, 21],
    [GPIO3, 0, 22],
    [GPIO4, 0, 23],
    // missing 0-24
    [PINT, 0, 25],
    [DMA0, 0, 26],
    [CRC, 0, 27],
    [WWDT0, 0, 28],
    [WWDT1, 0, 29],
    // missing 0-30
    [MailBox, 0, 31],
    [MRT, 1, 0],
    [OsTimer, 1, 1],
    [SCT, 1, 2],
    [ADC0, 1, 3],
    [ADC1, 1, 4],
    [DAC, 1, 5],
    [RTC, 1, 6],
    // missing 1-7
    [EVSIM0, 1, 8],
    [EVSIM1, 1, 9],
    [UTICK, 1, 10],
    [LPFlexCOMM0, 1, 11],
    [LPFlexCOMM1, 1, 12],
    [LPFlexCOMM2, 1, 13],
    [LPFlexCOMM3, 1, 14],
    [LPFlexCOMM4, 1, 15],
    [LPFlexCOMM5, 1, 16],
    [LPFlexCOMM6, 1, 17],
    [LPFlexCOMM7, 1, 18],
    [LPFlexCOMM8, 1, 19],
    [LPFlexCOMM9, 1, 20],
    [MICFIL, 1, 21],
    [Timer2, 1, 22],
    [USB0RAM, 1, 23],
    [USB0FSDCD, 1, 24],
    [USB0FS, 1, 25],
    [Timer0, 1, 26],
    [Timer1, 1, 27],
    // missing 1-28
    [PKCRAM, 1, 29],
    // missing 1-30
    [SmartDMA, 1, 31],
    [ESPI, 2, 0],
    [DMA1, 2, 1],
    [ENET, 2, 2],
    [USDHC, 2, 3],
    [FlexIO, 2, 4],
    [SAI0, 2, 5],
    [SAI1, 2, 6],
    [TRO, 2, 7],
    [FreqME, 2, 8],
    // missing 2-9 to 2-12
    [TRNG, 2, 13],
    [FlexCAN0, 2, 14],
    [FlexCAN1, 2, 15],
    [USBHS, 2, 16],
    [USBHSPHY, 2, 17],
    [CSS, 2, 18],
    [PowerQUAD, 2, 19],
    [PLULut, 2, 20],
    [Timer3, 2, 21],
    [Timer4, 2, 22],
    [PUF, 2, 23],
    [PKC, 2, 24],
    // missing 2-25
    [SCG, 2, 26],
    // missing 2-27, 2-28
    [GDET, 2, 29],
    [SM3, 2, 30],
    [I3C0, 3, 0],
    [I3C1, 3, 1],
    [SINC, 3, 2],
    [CoolFlux, 3, 3],
    [ENC0, 3, 4],
    [ENC1, 3, 5],
    [PWM0, 3, 6],
    [PWM1, 3, 7],
    [EVTG, 3, 8],
    // missing 3-9, 3-10
    [DAC1, 3, 11],
    [DAC2, 3, 12],
    [OPAMP0, 3, 13],
    [OPAMP1, 3, 14],
    [OPAMP2, 3, 15],
    // missing 3-16, 3-17
    [CMP2, 3, 18],
    [VREF, 3, 19],
    [CoolFluxAPB, 3, 20],
    [Neutron, 3, 21],
    [TSI, 3, 22],
    [EWM, 3, 23],
    [EIM, 3, 24],
    [ERM, 3, 25],
    [INTM, 3, 26],
    [SEMA42, 3, 27],
);

pub fn enable(peripheral: Peripherals) {
    peripheral.enable();
}

pub fn disable(peripheral: Peripherals) {
    peripheral.disable();
}
