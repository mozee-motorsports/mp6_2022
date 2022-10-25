use crate::drivers::asm_delay as ad;
use embassy_stm32::gpio as gpio;
use embassy_stm32::peripherals::*;

fn blink() {
    let pa5 = gpio::Output::new(PA5, Level::Low, Speed::Low);

    
}
















