// this is like a direct copy of turbo-resin/src/drivers/delay.rs
// we should make sure that cycle counts are correct
//
// clock speed is 16Mhz (62.5ns/cycle)
//

use embassy_time::delay::delay;
use cortex_m::asm;



const CLOCK_SPEED_MHZ: u32 = 16;

pub fn delay_blocking(delay: u32) {
    delay_us(delay);
}










