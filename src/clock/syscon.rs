use crate::pac;

// macro_rules! define_peripheral_clocks {
//     ( $( [$name:tt, $ahb:expr, $bit:expr] ), +, ) => {
//         // define_peripherals!(@define_enum $($name,)+);
//         #[derive(Debug, Clone, Copy)]
//         pub enum PeripheralClock {
//             $( $name, )+
//         }

//         impl PeripheralClock {
//             pub fn enable(peripheral: PeripheralClock) {
//                 let syscon = pac::SYSCON0::ptr();
//                 match peripheral {
//                     // $(define_peripherals!(@define_match_arm $name, $ahb, $bit))+
//                     $(
//                         PeripheralClock::$name => unsafe {
//                             (*syscon).ahbclkctrlset[$ahb].write(|w| w.bits(1 << $bit));
//                         },
//                     )+
//                 }
//             }

//             pub fn disable(peripheral: PeripheralClock) {
//                 let syscon = pac::SYSCON0::ptr();
//                 match peripheral {
//                     // $(define_peripherals!(@define_match_arm $name, $ahb, $bit))+
//                     $(
//                         PeripheralClock::$name => unsafe {
//                             (*syscon).ahbclkctrlclr[$ahb].write(|w| w.bits(1 << $bit));
//                         },
//                     )+
//                 }
//             }
//         }
//     };
// }

macro_rules! define_peripheral_clocks {
    ( $( [$name:tt, $ahb:expr, $bit:expr] ), +, ) => {
        #[non_exhaustive]
        pub struct PeripheralClock {
            ahb: u8,
            bit: u8,
        }

        impl PeripheralClock {
            $( pub const $name: PeripheralClock = PeripheralClock { ahb: $ahb, bit: $bit }; )+
        }
    };
}

define_peripheral_clocks!(
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
    [FLEX_SPI, 0, 11],
    [INPUT_MUX, 0, 12],
    [PORT0, 0, 13],
    [PORT1, 0, 14],
    [PORT2, 0, 15],
    [PORT3, 0, 16],
    [PORT4, 0, 17],
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
    [MAIL_BOX, 0, 31],
    [MRT, 1, 0],
    [OS_TIMER, 1, 1],
    [SCT, 1, 2],
    [ADC0, 1, 3],
    [ADC1, 1, 4],
    [DAC, 1, 5],
    [RTC, 1, 6],
    // missing 1-7
    [EVSIM0, 1, 8],
    [EVSIM1, 1, 9],
    [UTICK, 1, 10],
    [LPFLEXCOMM0, 1, 11],
    [LPFLEXCOMM1, 1, 12],
    [LPFLEXCOMM2, 1, 13],
    [LPFLEXCOMM3, 1, 14],
    [LPFLEXCOMM4, 1, 15],
    [LPFLEXCOMM5, 1, 16],
    [LPFLEXCOMM6, 1, 17],
    [LPFLEXCOMM7, 1, 18],
    [LPFLEXCOMM8, 1, 19],
    [LPFLEXCOMM9, 1, 20],
    [MICFIL, 1, 21],
    [TIMER2, 1, 22],
    [USB0RAM, 1, 23],
    [USB0FSDCD, 1, 24],
    [USB0FS, 1, 25],
    [TIMER0, 1, 26],
    [TIMER1, 1, 27],
    // missing 1-28
    [PKCRAM, 1, 29],
    // missing 1-30
    [SMARTDMA, 1, 31],
    [ESPI, 2, 0],
    [DMA1, 2, 1],
    [ENET, 2, 2],
    [USDHC, 2, 3],
    [FLEXIO, 2, 4],
    [SAI0, 2, 5],
    [SAI1, 2, 6],
    [TRO, 2, 7],
    [FREQME, 2, 8],
    // missing 2-9 to 2-12
    [TRNG, 2, 13],
    [FLEXCAN0, 2, 14],
    [FLEXCAN1, 2, 15],
    [USBHS, 2, 16],
    [USBHSPHY, 2, 17],
    [CSS, 2, 18],
    [POWERQUAD, 2, 19],
    [PLULUT, 2, 20],
    [TIMER3, 2, 21],
    [TIMER4, 2, 22],
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
    [COOLFLUX, 3, 3],
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
    [COOLFLUXAPB, 3, 20],
    [NEUTRON, 3, 21],
    [TSI, 3, 22],
    [EWM, 3, 23],
    [EIM, 3, 24],
    [ERM, 3, 25],
    [INTM, 3, 26],
    [SEMA42, 3, 27],
);

impl PeripheralClock {
    pub fn enable(peripheral: PeripheralClock) {
        let syscon = pac::SYSCON0::ptr();
        unsafe {
            (*syscon).ahbclkctrlset[peripheral.ahb as usize].write(|w| w.bits(1 << peripheral.bit));
        }
    }

    pub fn disable(peripheral: PeripheralClock) {
        let syscon = pac::SYSCON0::ptr();
        unsafe {
            (*syscon).ahbclkctrlclr[peripheral.ahb as usize].write(|w| w.bits(1 << peripheral.bit));
        }
    }
}
