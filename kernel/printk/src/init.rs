use novuskinc::drivers::names::CONSOLE;
use crate::{DEVICE_DRIVERS, PRINTK};

pub mod error {
    pub const SUCCESS: u8 = 0;
    pub const DRIVER_FAILED: u8 = 1;
    pub const DRIVER_NOT_FOUND: u8 = 2;
}

#[no_mangle]
pub unsafe extern "C" fn console_init() -> u8 {
    let console = DEVICE_DRIVERS.get_driver(CONSOLE);

    if console.is_some() {
        let console_ret = console.unwrap().init();

        if console_ret.is_err() {
            return error::DRIVER_FAILED;
        }

        PRINTK.set_init(true, console.unwrap());
        PRINTK.console_driver.unwrap().init();
        //PRINTK.console_driver.unwrap().write_character('a', 0, 0);
    } else { return error::DRIVER_NOT_FOUND; }

    return 0;
}
