use core::fmt::{Arguments, Write};
use printk::Printk;
use super::uart::Uart;

#[no_mangle]
pub extern "C" fn _early_printk(fmt: Arguments) {
    let mut uart = Uart::new();

    uart.write_fmt(fmt);
}

#[no_mangle]
pub extern "C" fn _kernel_main_print(fmt: Arguments) {

}

#[no_mangle]
pub static mut PRINTK: Printk = Printk {
    init: false,
    console_driver: None
};
