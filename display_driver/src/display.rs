use crate::*;
use embedded_graphics::{
    mono_font::{ascii::FONT_5X8, MonoTextStyle}, 
    prelude::*,
    text::Text,
};


pub struct Display {
    delay: SysDelay,
}
enum DisplayInstruction {
    Initialize,
    Clear, 
    SetPosition(u8, u8),
    PrintString(&'static str),
    PrintNum(u32),
}


impl Display {




    pub fn new(delay: SysDelay) -> Self {
        Display { delay, }
    } 

    pub fn wait_for(&mut self, time: u32) {
        self.delay.delay_ms(time);
    }
}

