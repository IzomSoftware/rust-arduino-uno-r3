use crate::types::*;
use avr_device::atmega328p::Peripherals;

#[allow(dead_code)]
pub fn digital_read(pin: &Pin) -> bool {
    let pin = pin.pin;
    let dp = unsafe { Peripherals::steal() };

    match pin {
        0 => dp.PORTD.pind.read().pd0().bit(),
        1 => dp.PORTD.pind.read().pd1().bit(),
        2 => dp.PORTD.pind.read().pd2().bit(),
        3 => dp.PORTD.pind.read().pd3().bit(),
        4 => dp.PORTD.pind.read().pd4().bit(),
        5 => dp.PORTD.pind.read().pd5().bit(),
        6 => dp.PORTD.pind.read().pd6().bit(),
        7 => dp.PORTD.pind.read().pd7().bit(),
        8 => dp.PORTB.pinb.read().pb0().bit(),
        9 => dp.PORTB.pinb.read().pb1().bit(),
        10 => dp.PORTB.pinb.read().pb2().bit(),
        11 => dp.PORTB.pinb.read().pb3().bit(),
        12 => dp.PORTB.pinb.read().pb4().bit(),
        13 => dp.PORTB.pinb.read().pb5().bit(),
        _ => panic!(),
    }
}

#[allow(dead_code)]
pub fn analog_read(pin: &Pin) -> u16 {
    let pin = pin.pin;
    let dp = unsafe { Peripherals::steal() };

    dp.ADC.admux.modify(|_, w| {
        w.refs().internal();
        unsafe { w.mux().bits(pin) }
    });
    dp.ADC.adcsra.modify(|_, w| w.adsc().set_bit());
    while dp.ADC.adcsra.read().adsc().bit_is_set() {}
    dp.ADC.adc.read().bits()
}
