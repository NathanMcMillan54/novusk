use core::fmt::{Arguments, Result};
use printk::PRINTK;
use crate::FB;

#[no_mangle]
pub unsafe extern "C" fn vga_write_fmt(args: Arguments) -> Result {

    Ok(())
}
