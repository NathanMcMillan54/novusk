use core::fmt::{Arguments, Write};
use novuskinc::drivers::names::CONSOLE;
use novuskinc::drivers::manager::DEVICE_DRIVERS;

pub fn early_console_write(fmt: Arguments) {
    let mut console_driver = unsafe { DEVICE_DRIVERS.get_driver(CONSOLE).unwrap() };
    console_driver.write_fmt(fmt);
}

#[macro_export]
macro_rules! early_printk {
    ($($args:tt)*) => {
        x86_64::instructions::interrupts::without_interrupts(|| {
            $crate::kernel::early_console::early_console_write(format_args!($($args)*));
        });
    };
}
