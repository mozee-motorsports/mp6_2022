/*
 * this is our unsafe api with direct memory access
 */


use cortex_m::{register::faultmask::read, asm::delay};
use hal::timer::DelayUs;
pub use stm32f4xx_hal as hal;
pub use hal::{pac, prelude::*};
use core::ptr::{write_volatile, read_volatile};

const GPIOA_BSRR: *mut u32 = 0x40020018 as *mut u32;
const GPIOC_BSRR: *mut u32 = 0x40020818 as *mut u32;


const GPIOA_ODR: *mut u32 = 0x40020014 as *mut u32;
const GPIOC_ODR: *mut u32 = 0x40020814 as *mut u32;


pub struct DisplayDriver {
    delay: hal::timer::TimerExt<Self>,
}

impl DisplayDriver  {

    pub fn new(delay: hal::pac::TIM5) -> Self {
        DisplayDriver {
            delay,
        }
    }

    fn write_instruction(&self, instruction: u8) -> Result<(), ()> {
        let instruction_mask: u32 = instruction as u32;

        let imask: u32 = 0b111 << 24; 
        let e_set: u32 = 0b1 << 10;
        let e_clr: u32 = &e_set << 16;

        unsafe {
            write_volatile(GPIOA_BSRR, imask );
            write_volatile(GPIOA_BSRR, e_set );
            write_volatile(GPIOA_ODR, (read_volatile(GPIOA_ODR) & 0b000000001111) | instruction_mask); 
            write_volatile(GPIOA_BSRR, e_set);
        }

        self.delay.delay_us(40_u8);

        Ok(())
    }


}

