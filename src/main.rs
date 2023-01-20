#![no_std]
#![no_main]

use atmega_hal::adc;
use atmega_hal::clock::{Clock, MHz16};
use atmega_hal::prelude::*;
use atmega_hal::usart::Baudrate;
// use embedded_hal::serial::Read;
// use heapless::String;
// use heapless::Vec;
use panic_halt as _;

#[no_mangle]
pub extern "C" fn main() {
    let dp = atmega_hal::Peripherals::take().unwrap();
    let pins = atmega_hal::pins!(dp);
    // let mut serial = atmega_hal::default_serial!(dp, pins, 57600);

    let brate: Baudrate<MHz16> = Baudrate::new(56700);
    let mut serial = atmega_hal::Usart::new(dp.USART0, pins.pd0, pins.pd1.into_output(), brate);

    let mut led = pins.pb5.into_output();

    let mut delay = atmega_hal::delay::Delay::<MHz16>::new();
    loop {
        let t1: u16 = 200;
        delay.delay_ms(t1);
        led.toggle();
        ufmt::uwriteln!(&mut serial, "Hello, world!").unwrap();
    }
}
