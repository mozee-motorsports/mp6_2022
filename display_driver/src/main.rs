#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]


mod display;
use display::*;

use hal::timer::SysDelay;
use panic_halt as _;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;
use crate::hal::{pac, prelude::*};

pub use embedded_hal::blocking::delay::{DelayMs, DelayUs};



const GPIOA_BSRR: *mut u32 = 0x40020018 as *mut u32;
const GPIOC_BSRR: *mut u32 = 0x40020818 as *mut u32;
const GPIOA_ODR: *mut u32 = 0x40020014 as *mut u32;
const GPIOC_ODR: *mut u32 = 0x40020814 as *mut u32;
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

    let rcc = device.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

    let display = Display::new(chip.SYST.delay(&clocks));



    loop {

    }

}

pub fn init() -> (pac::Peripherals, cortex_m::Peripherals) {
    if let (Some(device_peripherals), Some(chip_peripherals)) = 
        (pac::Peripherals::take(), cortex_m::Peripherals::take(),) {
            (device_peripherals, chip_peripherals,)
    } else { panic!() }
}


