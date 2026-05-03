use crate::info;
use crate::serial::get_serial;
use avr_device::atmega328p::Peripherals;

pub struct OnPinDigitalWriteEvent {
    pub dp: Peripherals,
    pub pin: u8,
    pub data: bool,
}

fn t() {
    get_serial(|serial| {
        info!(serial, "kos zanet sina");
    })
}

impl OnPinDigitalWriteEvent {
    pub fn new(dp: Peripherals, pin: u8, data: bool) -> Self {
        OnPinDigitalWriteEvent { dp, pin, data }
    }
    pub fn get_pin(&self) -> u8 {
        self.pin
    }
    pub fn get_data(&self) -> bool {
        self.data
    }
    pub fn call(&self) {
        t();
    }
}
