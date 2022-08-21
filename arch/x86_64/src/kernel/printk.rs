use core::fmt::{Arguments, Result};
use super::vga::_vga_print;
use printk::Printk;
// use gpu::GpuGraphics;

#[no_mangle]
pub extern "C" fn _early_printk(fmt: Arguments) -> Result {
    _vga_print(fmt);
    Ok(())
}

#[no_mangle]
pub static mut PRINTK: Printk = Printk {
    init: false,
    console_driver: None,
};

#[export_name = "_kernel_main_print"]
#[no_mangle]
pub extern "C" fn kernel_main_print(x: usize, y: usize, fmt: Arguments) {
    // let mut gpu = GpuGraphics::new();

    // gpu.graphics_print(x, y, 15, fmt);
}
