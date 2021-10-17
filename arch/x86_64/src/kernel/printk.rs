use core::fmt::Arguments;
// use gpu::GpuGraphics;

#[export_name = "_kernel_main_print"]
#[no_mangle]
pub extern "C" fn kernel_main_print(x: usize, y: usize, fmt: Arguments) {
    // let mut gpu = GpuGraphics::new();

    // gpu.graphics_print(x, y, 15, fmt);
}
