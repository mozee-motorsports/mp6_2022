#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};


#[entry]
fn main() -> !{
    if let (Some(dp), Some(cp)) = (pac::Peripherals::take(), cortex_m::Peripherals::take(),)
    {
        let gpioa = dp.GPIOA.split();
        let mut led = gpioa.pa5.into_push_pull_output();

        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

        let mut delay = cp.SYST.delay(&clocks);

        loop {
            led.set_high();
            delay.delay_ms(2000_u32);
            led.set_low();
            delay.delay_ms(2000_u32);
        }

    }
    loop {}

}
