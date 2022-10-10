#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;
pub use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};

mod lcd_api;
use lcd_api::*;
/*
 * Notes: 
 * Display Data bus is on PA4-11
 * Control on PC8-10
 */ 


#[entry]
fn main() -> !{
    let peripherals = init();
    let device = peripherals.0;
    let chip = peripherals.1;

    loop {}

}

pub fn init() -> (pac::Peripherals, cortex_m::Peripherals) {
    if let (Some(device_peripherals), Some(chip_peripherals)) = 
        (pac::Peripherals::take(), cortex_m::Peripherals::take(),) {
            (device_peripherals, chip_peripherals,)
    } else { panic!() }
}

