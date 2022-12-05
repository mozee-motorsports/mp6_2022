#![no_std]
#![no_main]

use cortex_m::asm::delay;
use drivers::delay::delay_blocking;
use embassy_stm32::dma::NoDma;
use embassy_stm32::time::Hertz;
use embassy_time::Delay;
use cortex_m_rt as rt;

mod drivers;
mod consts;


#[rt::entry]
fn main() -> ! {

    let p = embassy_stm32::init(Default::default());
    let mut led = Output::new(p.PA5, Level::High, Speed::Low); 

    loop {
        led.set_high(); 
        drivers.delay::delay_blocking(1000_u32);
        led.set_low(); 
        drivers.delay::delay_blocking(1000_u32);
    }
}
