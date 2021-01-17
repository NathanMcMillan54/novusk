use super::startKernel;
use super::super::kernel::{init::{boot_msg, x86_init}, vga_buffer::Color::White};

#[no_mangle]
pub extern "C" fn initStart() -> ! {
    boot_msg("Starting...\n", 0, White);
    unsafe { x86_init() }
}
