/*
 * this is our unsafe api with direct memory access
 */


pub use stm32f4xx_hal as hal;
pub use hal::{pac, prelude::*};

const RCC_BASE: *const i32 = 0x40023800 as *const i32;
const GPIOA_BASE: *const i32 = 0x40020000 as *const i32;
const GPIOC_BASE: *const i32 = 0x40020800 as *const i32;

pub struct Databus {
    bsrr: hal::gpio::gpioc::bsrr,
}





pub struct Display {
    device_peripherals: pac::Peripherals,
    chip_peripherals: cortex_m::Peripherals,
}

pub impl Display {
    fn from(() 

}

