#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;

use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use embedded_hal::digital::v2::OutputPin;


use stm32f4xx_hal as hal;
use crate::hal::{pac, prelude::*};

const GPIOA_BSRR: *mut u32 = 0x40020018 as *mut u32;
const GPIOC_BSRR: *mut u32 = 0x40020818 as *mut u32;
const GPIOA_ODR: *mut u32 = 0x40020014 as *mut u32;
const GPIOC_ODR: *mut u32 = 0x40020814 as *mut u32;
/*
 * Notes: 
 * Display Data bus is on PA4-11
 * Control on PC8-10
 */ 

struct Display<

    pub fn reset<RST, DELAY>(&mut self, rst: &mut RST, delay: &mut DELAY)
        where
            RST: OutputPin,
            DELAY: DelayMs<u8>,
        {
            rst.set_high().ok().unwrap();
            delay.delay_ms(100);

            rst.set_low().ok().unwrap();
            delay.delay_ms(100);

            rst.set_high().ok().unwrap();
            delay.delay_ms(100);
    }


#[entry]
fn main() -> !{
    let peripherals = init();

    let device: pac::Peripherals = peripherals.0;
    let chip = peripherals.1;

    // let rcc = device.RCC.constrain();
    // let clocks = rcc.cfgr.sysclk(48.Mhz()).freeze();

    // let mut delay = chip.SYST.delay(&clocks);

    // let driver = DisplayDriver::new(delay.clone());

    let gpioa = device.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();

    let rcc = device.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

    let mut delay = chip.SYST.delay(&clocks);


    loop {
        led.set_high();
        delay.delay_ms(2000_u32);
        led.set_low();
        delay.delay_ms(2000_u32);
    }

}

pub fn init() -> (pac::Peripherals, cortex_m::Peripherals) {
    if let (Some(device_peripherals), Some(chip_peripherals)) = 
        (pac::Peripherals::take(), cortex_m::Peripherals::take(),) {
            (device_peripherals, chip_peripherals,)
    } else { panic!() }
}


