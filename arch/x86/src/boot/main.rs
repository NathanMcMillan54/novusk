use crate::kernel::init::x86_kernel_init;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    crate::vga_write!("Starting kernel...");

    x86_kernel_init();
}
