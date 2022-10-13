const GPIOA_BSRR: *mut u32 = 0x40020018 as *mut u32;
const GPIOC_BSRR: *mut u32 = 0x40020818 as *mut u32;
const GPIOA_ODR: *mut u32 = 0x40020014 as *mut u32;
const GPIOC_ODR: *mut u32 = 0x40020814 as *mut u32;

use crate::*;
use hal::pac::{GPIOC, GPIOA};
use stm32f4xx_hal as hal;


pub struct Display {
    delay: SysDelay,
    ctrl_bus: hal::pac::GPIOC,
    data_bus: hal::pac::GPIOA,
}

enum DisplayInstruction {
    Initialize,
    Clear, 
    SetPosition(u8, u8),
    PrintString(&'static str),
    PrintNum(u32),
}


impl Display {
    const RS: u32 = 0x8;
    const RW: u32 = 0x9;
    const E: u32 = 0x10;

    const RS_SET: u32 = 0b1 << RW;



    pub fn new(delay: SysDelay, ctrl_bus: GPIOC, data_bus: GPIOA) -> Self {
        Display { delay, ctrl_bus, data_bus}
    } 

    pub fn wait_for(&mut self, time: u32) {
        self.delay.delay_ms(time);
    }

    fn write_data(&mut self, data: u32) {
        self.ctrl_bus.bsrr.write(|w| unsafe { w.bits()


        self.data_bus.odr.modify(|r, w| unsafe {
            w.bits(data << 4 | (r.bits()) & 0b111111110000) 
        });


    }
}

