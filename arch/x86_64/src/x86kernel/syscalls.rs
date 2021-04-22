use core::fmt::{Arguments, Write};
use drivers::x86_64::vga::WRITER;
use super::time::current_time;

#[no_mangle]
pub unsafe extern "C" fn write(bytes: &[u8], size: u8) -> u8 {
    WRITER.lock().write_byte(*bytes.as_ptr());
    return size;
}

#[no_mangle]
pub unsafe extern "C" fn write_fmt(args: Arguments) -> Arguments {
    WRITER.lock().write_fmt(args).unwrap();
    return args;
}

#[no_mangle]
pub unsafe extern "C" fn kernel_time() -> f64 {
    current_time()
}
