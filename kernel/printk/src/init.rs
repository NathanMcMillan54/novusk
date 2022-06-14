use novuskinc::drivers::names::CONSOLE;
use crate::{DEVICE_DRIVERS, PRINTK};

#[no_mangle]
pub unsafe extern "C" fn console_init() {
    let console = DEVICE_DRIVERS.get_driver(CONSOLE);

    if console.is_some() {
        let console_ret = console.unwrap().init().unwrap();

        if console_ret.is_err() {
            panic!("{}", console_ret.err().unwrap_or("Unknown cause"));
        }

        PRINTK.set_init(true, console.unwrap());
    } else { panic!("Could not find Console Driver"); }
}
