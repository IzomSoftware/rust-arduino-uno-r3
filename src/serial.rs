use atmega_hal::port::{PD0, PD1};
use atmega_hal::{Atmega, pins};
use avr_device::atmega328p::{Peripherals, USART0};
use avr_hal_generic::clock::MHz16;
use avr_hal_generic::port;
use avr_hal_generic::port::mode::{Input, Output};
use avr_hal_generic::usart::{Baudrate, Usart};
use core::cell::OnceCell;
use critical_section::Mutex;

type Serial = Usart<Atmega, USART0, port::Pin<Input, PD0>, port::Pin<Output, PD1>, MHz16>;

pub struct MarkSync<T>(pub T);
unsafe impl<T> Sync for MarkSync<T> {}

static SERIAL: MarkSync<
    OnceCell<
        Mutex<
            core::cell::RefCell<
                Serial,
            >,
        >,
    >,
> = MarkSync(OnceCell::new());
pub fn init_serial() {
    let dp = unsafe { Peripherals::steal() };
    let ports = pins!(dp);
    let _ = SERIAL.0.set(Mutex::new(core::cell::RefCell::new(Usart::new(
        dp.USART0,
        ports.pd0,
        ports.pd1.into_output(),
        Baudrate::<MHz16>::new(57600),
    ))));
}

pub fn get_serial<F, R>(f: F) -> R
where
    F: FnOnce(
        &mut Serial,
    ) -> R,
{
    let cell = &SERIAL.0;
    let mutex = cell.get().expect("Serial not initialized!");
    critical_section::with(|cs| f(&mut mutex.borrow(cs).borrow_mut()))
}

#[macro_export]
macro_rules! info {
    (
        $serial:expr, $s:expr
    ) => {
        ufmt::uwriteln!($serial, $s).unwrap();
    };
}
