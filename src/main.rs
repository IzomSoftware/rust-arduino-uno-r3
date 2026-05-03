#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

mod delay;
mod events;
mod pin_mode;
mod reader;
mod serial;
mod types;
mod writer;

use crate::delay::delay;
use crate::pin_mode::pin_mode;
use crate::serial::{get_serial, init_serial};
use crate::types::{Mode, Pin, Type};
use crate::writer::{digital_write, enable_analog};
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    // let dp = Peripherals::take().unwrap();
    // let pins = pins!(dp);

    // Enable ADC
    enable_analog();

    init_serial();

    delay(100);
    // Setup Serial
    get_serial(|serial| {
        info!(serial, "Starting up ATMEGA328P");
    });

    let led2 = Pin { pin: 2 };

    pin_mode(Type::Digital, Mode::Output, &led2);
    digital_write(&led2, true);

    loop {}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    panic!()
}
