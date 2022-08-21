use core::fmt::{Arguments, Result, Write};
use printk::Printk;
use super::io::IO;

#[no_mangle]
pub extern "C" fn _early_printk(fmt: Arguments) -> Result {
    unsafe { IO.write_fmt(fmt) }
}

#[no_mangle]
pub extern "C" fn _kernel_main_print(fmt: Arguments) {

}

#[no_mangle]
pub static mut PRINTK: Printk = Printk {
    init: false,
    console_driver: None
};
