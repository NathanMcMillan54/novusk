use arm::include::asm;
use crate::boot::msg::boot_msg;
use super::cmdline;
use crate::sleep;

pub unsafe fn aarch64_init() -> ! {
    sleep(1);
    cmdline::cmdline_init();
    sleep(1);
    boot_msg("Cmdline initialized\n", 0);
    aarch64_end_kernel()
}

#[no_mangle]
pub extern "C" fn aarch64_end_kernel() -> ! {
    unsafe { asm::wfe() }
}
