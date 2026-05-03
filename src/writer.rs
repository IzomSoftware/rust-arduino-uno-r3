use crate::events::OnPinDigitalWriteEvent;
use crate::types::*;
use avr_device::atmega328p::Peripherals;

#[allow(dead_code)]
pub fn digital_write(pin: &Pin, val: bool) {
    let pin = pin.pin;
    let dp = unsafe { Peripherals::steal() };

    let event = OnPinDigitalWriteEvent::new(dp, pin, val);

    event.call();

    let dp = unsafe { Peripherals::steal() };

    match pin {
        0 => dp.PORTD.portd.modify(|_, w| w.pd0().bit(val)),
        1 => dp.PORTD.portd.modify(|_, w| w.pd1().bit(val)),
        2 => dp.PORTD.portd.modify(|_, w| w.pd2().bit(val)),
        3 => dp.PORTD.portd.modify(|_, w| w.pd3().bit(val)),
        4 => dp.PORTD.portd.modify(|_, w| w.pd4().bit(val)),
        5 => dp.PORTD.portd.modify(|_, w| w.pd5().bit(val)),
        6 => dp.PORTD.portd.modify(|_, w| w.pd6().bit(val)),
        7 => dp.PORTD.portd.modify(|_, w| w.pd7().bit(val)),

        8 => dp.PORTB.portb.modify(|_, w| w.pb0().bit(val)),
        9 => dp.PORTB.portb.modify(|_, w| w.pb1().bit(val)),
        10 => dp.PORTB.portb.modify(|_, w| w.pb2().bit(val)),
        11 => dp.PORTB.portb.modify(|_, w| w.pb3().bit(val)),
        12 => dp.PORTB.portb.modify(|_, w| w.pb4().bit(val)),
        13 => dp.PORTB.portb.modify(|_, w| w.pb5().bit(val)),
        _ => panic!(),
    }
}

#[allow(dead_code)]
pub fn enable_analog() {
    let dp = unsafe { Peripherals::steal() };

    dp.ADC
        .adcsra
        .modify(|_, w| w.aden().set_bit().adps().prescaler_128());
    dp.ADC.admux.modify(|_, w| w.refs().avcc());
}

#[allow(dead_code)]
pub fn analog_write(pin: &Pin, val: u8) {
    let pin = pin.pin;
    let dp = unsafe { Peripherals::steal() };

    let cport = &dp.PORTC.portc;

    match pin {
        0 => cport.modify(|_, w| unsafe { w.pc0().set_bit().bits(val) }),
        1 => cport.modify(|_, w| unsafe { w.pc1().set_bit().bits(val) }),
        2 => cport.modify(|_, w| unsafe { w.pc2().set_bit().bits(val) }),
        3 => cport.modify(|_, w| unsafe { w.pc3().set_bit().bits(val) }),
        4 => cport.modify(|_, w| unsafe { w.pc4().set_bit().bits(val) }),
        5 => cport.modify(|_, w| unsafe { w.pc5().set_bit().bits(val) }),
        _ => panic!(),
    }
}
