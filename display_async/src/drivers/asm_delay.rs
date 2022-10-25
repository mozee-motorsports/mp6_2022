// this is like a direct copy of turbo-resin/src/drivers/delay.rs
// we should make sure that cycle counts are correct
//
// clock speed is 16Mhz (62.5ns/cycle)
//

use cortex_m::asm;
use crate::drivers::asm_delay as asm_delay;
use crate::consts as consts;


#[inline(always)]
pub fn delay_cycles(cycles: u32) {
    // The official crate overshoots on Cortex-M4
    let cycles = (cycles*2)/3;
    asm::delay(cycles);
}

pub fn delay_us(delay_us: u32) {
    // 16 cycles/ 1ms
    let cycles = delay_us * consts::CLOCK_SPEED_MHZ;
    asm_delay::delay_cycles(cycles);
}
    
pub fn delay_ms(delay_ms: u32) {
    let microseconds = delay_ms * 1000;
    asm_delay::delay_us(microseconds);
}
    
pub fn delay_s(delay_ms: u32) {
    let seconds = delay_ms * 1000;
    asm_delay::delay_ms(seconds as u32);
}










