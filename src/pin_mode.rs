use crate::types::*;
use avr_device::atmega328p::Peripherals;

// mode -> true = OUTPUT, false = INPUT
pub fn pin_mode(r#type: Type, mode: Mode, pin: &Pin) {
    let pin = pin.pin;
    let dp = unsafe { Peripherals::steal() };

    match r#type {
        Type::Digital => {
            let ddrd = &dp.PORTD.ddrd;
            let ddrb = &dp.PORTB.ddrb;

            match pin {
                // 0 - 7
                0 => ddrd.modify(|_, w| w.pd0().bit(mode.get_val())),
                1 => ddrd.modify(|_, w| w.pd1().bit(mode.get_val())),
                2 => ddrd.modify(|_, w| w.pd2().bit(mode.get_val())),
                3 => ddrd.modify(|_, w| w.pd3().bit(mode.get_val())),
                4 => ddrd.modify(|_, w| w.pd4().bit(mode.get_val())),
                5 => ddrd.modify(|_, w| w.pd5().bit(mode.get_val())),
                6 => ddrd.modify(|_, w| w.pd6().bit(mode.get_val())),
                7 => ddrd.modify(|_, w| w.pd7().bit(mode.get_val())),
                // 8 - 13
                8 => ddrb.modify(|_, w| w.pb0().bit(mode.get_val())),
                9 => ddrb.modify(|_, w| w.pb1().bit(mode.get_val())),
                10 => ddrb.modify(|_, w| w.pb2().bit(mode.get_val())),
                11 => ddrb.modify(|_, w| w.pb3().bit(mode.get_val())),
                12 => ddrb.modify(|_, w| w.pb4().bit(mode.get_val())),
                13 => ddrb.modify(|_, w| w.pb5().bit(mode.get_val())),
                // unknown
                _ => panic!(),
            }
        }
        Type::Analog => match mode {
            Mode::Output => {
                let ddrc = &dp.PORTC.ddrc;

                match pin {
                    0 => ddrc.modify(|_, w| w.pc1().bit(true)),
                    1 => ddrc.modify(|_, w| w.pc1().bit(true)),
                    2 => ddrc.modify(|_, w| w.pc2().bit(true)),
                    3 => ddrc.modify(|_, w| w.pc3().bit(true)),
                    4 => ddrc.modify(|_, w| w.pc4().bit(true)),
                    5 => ddrc.modify(|_, w| w.pc5().bit(true)),
                    _ => panic!(),
                }
            }
            _ => panic!(),
        },
    }
}
