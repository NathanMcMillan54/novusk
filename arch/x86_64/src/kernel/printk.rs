use core::fmt::Arguments;
use printk::Printk;
// use gpu::GpuGraphics;

#[no_mangle]
pub static mut PRINTK: Printk = Printk::new();

#[export_name = "_kernel_main_print"]
#[no_mangle]
pub extern "C" fn kernel_main_print(x: usize, y: usize, fmt: Arguments) {
    // let mut gpu = GpuGraphics::new();

    // gpu.graphics_print(x, y, 15, fmt);
}
