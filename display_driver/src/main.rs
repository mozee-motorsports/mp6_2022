#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]


mod display;
mod tim5;

use display::*;
use tim5::*;

use hal::timer::SysDelay;
use panic_halt as _;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;
use crate::hal::{pac, prelude::*};

pub use embedded_hal::blocking::delay::{DelayMs, DelayUs};

use embedded_graphics::{
    mono_font::{ascii::FONT_5X8, MonoTextStyle}, 
    prelude::*,
    text::Text,
};


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

    let gpioc = device.GPIOC;
    let gpioa = device.GPIOA;

    let display = Display::new(chip.SYST.delay(&clocks), gpioc, gpioa);
    let display_2 = chip.SYST.delay(&clocks);



    loop {

    }

}

pub fn init() -> (pac::Peripherals, cortex_m::Peripherals) {
    if let (Some(device_peripherals), Some(chip_peripherals)) = 
        (pac::Peripherals::take(), cortex_m::Peripherals::take(),) {
            (device_peripherals, chip_peripherals,)
    } else { panic!() }
}


