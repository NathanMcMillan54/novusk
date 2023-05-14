use novuskinc::dif::{get_dif_value};
use novuskinc::drivers::{add_driver, Driver};
use novuskinc::drivers::names::SIMPLE_UART;
use tm4c123x_hal::Peripherals;

pub unsafe fn common_early_stellaris_init() {
    if Peripherals::take().is_none() {
        panic!("Device peripherals can not be found, Novusk might be running on an unsupported device");
    }

    //if get_dif_value("PrintingMethod").contains(SIMPLE_UART) {
        //#[cfg(feature = "stellaris_6965")]
        super::s6965::uart::lm3s6965_simpleuart_init();

        unsafe { core::ptr::write_volatile(0x4000_C000 as *mut u8, b'h'); }
    //} else {
        //add_driver(&HioDriver as &'static dyn Driver);
    //}
}
