use core::fmt::{Arguments, Write};
use super::io::IO;

#[no_mangle]
pub extern "C" fn arch_printk(fmt: Arguments) {
    unsafe { IO.write_fmt(fmt); }
}


