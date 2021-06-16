use crate::kernel::init::x86_kernel_init;
use crate::kernel::printk::boot_method;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    crate::vga_write!("Starting kernel...");
    crate::vga_write!("Boot method: {}", boot_method());

    x86_kernel_init();
}
