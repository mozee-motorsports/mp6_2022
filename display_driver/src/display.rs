const GPIOA_BSRR: *mut u32 = 0x40020018 as *mut u32;
const GPIOC_BSRR: *mut u32 = 0x40020818 as *mut u32;
const GPIOA_ODR: *mut u32 = 0x40020014 as *mut u32;
const GPIOC_ODR: *mut u32 = 0x40020814 as *mut u32;

//TODO
use crate::*;

use hal::pac::{GPIOC, GPIOA};
use stm32f4xx_hal as hal;

use embedded_graphics::{
    mono_font::{ascii::FONT_5X8, MonoTextStyle}, 
    prelude::*,
    text::Text,
};

pub struct Display {
    delay: SysDelay,
    ctrl_bus: hal::pac::GPIOC,
    data_bus: hal::pac::GPIOA,
}

// enum DisplayInstruction {
//     Initialize,
//     Clear, 
//     SetPosition(u32, u32),
//     PrintString(&'static str),
//     PrintNum(u32),
// }

enum DisplayInstruction {
    Clear, 
    Home, 
    EntryMode,
    OnOff,
    Shift(Direction),
    FunctionSet, 
    SetCGRAM,
    SetDDRAM, 
    ReadAddress, 
    ReadBusyFlag,
    // ReadData,
}

enum Direction {
    Left,
    Right,
}

use Direction as _;

enum DisplayError {
    Error,
}


impl Display {
    const RS_SET: u32 = 0b1 << 8;
    const RS_CLR: u32 = 0b1 << 24;

    const RW_SET: u32 = 0b1 << 9;
    const RW_CLR: u32 = 0b1 << 25;

    const E_SET: u32 = 0b1 << 10;
    const E_CLR: u32 = 0b1 << 26;

    pub fn new(delay: SysDelay, ctrl_bus: GPIOC, data_bus: GPIOA) -> Self {
        Display { delay, ctrl_bus, data_bus}
    } 

    pub fn wait_for(&mut self, time: u32) {
        self.delay.delay_ms(time);
    }

    fn write_data(&mut self, data: u32) -> Result<(), DisplayError> {
        let mask: u32 = Display::RS_SET | Display::RW_CLR | Display::E_CLR;
        self.ctrl_bus.bsrr.write(|w| unsafe { w.bits(mask) }); 

        // Strobe Enable
        self.ctrl_bus.bsrr.write(|w| unsafe { w.bits(Display::E_SET) }); 

        // Write out to data bus
        self.data_bus.odr.modify(|r, w| unsafe {
            w.bits(data << 4 | (r.bits()) & 0b111111110000) 
        });

        self.delay.delay_us(40_u32);

        Ok(())
    }

    fn write_instruction(&mut self, instruction: DisplayInstruction) -> Result<(), DisplayError> {
        let mask: u32 = Display::RS_SET | Display::RW_CLR | Display::E_CLR;
        self.ctrl_bus.bsrr.write(|w| unsafe { w.bits(mask) }); 

        let data: u32 = match instruction {
            DisplayInstruction::Clear => 0x01,
            DisplayInstruction::Home => 0x02,
            DisplayInstruction::FunctionSet => 0x38,
            DisplayInstruction::EntryMode => 0x06,
            DisplayInstruction::OnOff => 0x0F, 
            _ => 0,  // FIX: Fix this
        };
            
        // Strobe Enable
        self.ctrl_bus.bsrr.write(|w| unsafe { w.bits(Display::E_SET) }); 

        self.data_bus.odr.modify(|r, w| unsafe {
            w.bits(data << 4 | (r.bits()) & 0b111111110000) 
        });

        self.delay.delay_ms(10);            // HACK: too long, shouldn't be hardcoded

        Ok(())
    }

    pub fn lcd_init(&mut self) {
        self.delay.delay_ms(40_u32);
        self.write_instruction(DisplayInstruction::FunctionSet);
        self.write_instruction(DisplayInstruction::FunctionSet);
        self.write_instruction(DisplayInstruction::OnOff);
        self.write_instruction(DisplayInstruction::Clear);
        self.write_instruction(DisplayInstruction::EntryMode);
    }

    pub fn lcd_set_position(&mut self, row: bool, col: u32) {
        self.write_instruction(DisplayInstruction::Home);
        if row {
            for i in 0..40 {
                self.write_instruction(DisplayInstruction::Shift(Direction::Right));
            }
        }

        for i in 0..col {
            self.write_instruction(DisplayInstruction::Shift(Direction::Right));
        }
    }

    pub fn print_string(&mut self, string: &str) {
        // HACK: hacky solution, should check if string contains non-ASCII character


        let str = string.bytes()
            .map(|x| self.write_data(x_u32));
    }



}
