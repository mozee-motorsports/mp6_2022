use crate::drivers::asm_delay as ad;
use embassy_stm32::gpio as gpio;
use embassy_stm32::peripherals::*;
use stm32f4::stm32f411 as pac;
use pac::GPIOA;

struct Display{}

impl Display {
    fn write_data() {
        unsafe { (GPIOA::ptr()).idr.read().bits & 1 != 0}



    }














