#![no_std]
#![no_main]

use core::panic::PanicInfo;
use servo::*;

#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    loop {}
}

#[arduino_hal::entry]
fn main() -> ! {
    let peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(peripherals);

    let mut led = pins.d13.info_output();


    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}

