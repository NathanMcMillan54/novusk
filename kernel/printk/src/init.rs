use core::fmt::Write;
use novuskinc::drivers::names::{CONSOLE, SERIAL};
use crate::{DEVICE_DRIVERS, PRINTK};

pub mod error {
    pub const SUCCESS: u8 = 0;
    pub const DRIVER_FAILED: u8 = 1;
    pub const DRIVER_NOT_FOUND: u8 = 2;
}

#[no_mangle]
pub unsafe extern "C" fn printk_init() -> u8 {
    let console = DEVICE_DRIVERS.get_driver(CONSOLE);

    let console_ret = console.unwrap().init();

    if console_ret.is_err() {
        return error::DRIVER_FAILED;
    }

    PRINTK.set_init(true, console.unwrap());
    PRINTK.console_driver.unwrap().init();
    //PRINTK.console_driver.unwrap().write_character('a', 0, 0);
    //PRINTK.write_fmt(format_args!("{}", "test\n"));

    return 0;
}
