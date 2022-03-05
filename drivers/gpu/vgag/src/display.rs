use core::fmt::{Arguments, Result};
use printk::PrintK;
use spin::Mutex;
use crate::FB;

#[no_mangle]
pub extern "C" fn vga_write_fmt(args: Arguments) -> Result {
    unsafe { FB.graphics.graphics_write_fmt(args); }
    Ok(())
}

set_printk!(vga_write_fmt);
