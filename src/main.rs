#![no_std]
#![no_main]

use ruduino::cores::current::port;
use ruduino::Pin;

#[no_mangle]
pub extern "C" fn main() {
    port::B5::set_output();

    loop {
        port::B5::set_high();

        ruduino::delay::delay_ms(1000);

        port::B5::set_low();

        ruduino::delay::delay_ms(1000);
    }
}
