use super::boot::{boot_method, device_name};
use crate::kernel::init::x86_kernel_init;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    printk!("Starting kernel...");
    printk!("Device info:");
    printk!("    Boot method: {}", boot_method());
    printk!("    Device name: {}", device_name());

    x86_kernel_init();
}
