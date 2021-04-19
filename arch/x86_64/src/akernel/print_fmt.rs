use core::fmt::{Arguments, Write};
use core::ptr::Unique;
use super::vga::WRITER;

#[no_mangle]
pub extern "C" fn _x86_print(args: Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}
