#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;

use mcx_hal::{self as hal, pac};

#[entry]
fn main() -> ! {
    loop {}
}
