use core::fmt::Arguments;
use gpu::print::graphics_print;

#[export_name = "kmain_print"]
#[no_mangle]
pub extern "C" fn main_kernel_printk(args: Arguments) {
    graphics_print(0, 0, 15, args);
}
