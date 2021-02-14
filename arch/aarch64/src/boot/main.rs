use crate::drivers::uart::UART0;
use crate::kernel::init::aarch64_init;
use super::msg::boot_msg;

#[no_mangle]
pub extern "C" fn initStart() -> ! {
    boot_msg("Booting...\n", 0);
    boot_msg("\nStarting", 0);
    boot_msg("Novusk...\n", 8);
    boot_msg("v1.0.0 New Kernel\n", 0);
    unsafe { aarch64_init() }
}
