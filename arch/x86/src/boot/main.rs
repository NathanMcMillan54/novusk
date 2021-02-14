use super::{msg::boot_msg, startKernel};
use crate::kernel::{init::x86_init, vga_buffer::Color::*};

#[no_mangle]
pub extern "C" fn initStart() -> ! {
    boot_msg("Booting...\n", 0, White);
    boot_msg("\nStarting", 0, White);
    boot_msg(" Novusk...\n", 8, Cyan);
    boot_msg("v1.0.0 New Kernel", 0, Cyan);
    unsafe { x86_init() }
}
