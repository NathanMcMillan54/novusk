use crate::drivers::uart::UART0;
use crate::kernel::init::aarch64_init;
use super::msg::boot_msg;

#[no_mangle]
pub extern "C" fn initStart() -> ! {
    boot_msg("Booting...\n", 0);
    unsafe { aarch64_init() }
}
