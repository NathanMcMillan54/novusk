use arm::include::asm;
use crate::boot::msg::boot_msg;
use super::{cmdline, time::time};
use crate::sleep;

pub unsafe fn aarch64_init() -> ! {
    sleep(1);
    time::time_init();
    boot_msg("Time initialized\n", 0);
    cmdline::cmdline_init();
    boot_msg("Cmdline initialized\n", 0);
    aarch64_end_kernel()
}

#[no_mangle]
pub extern "C" fn aarch64_end_kernel() -> ! {
    unsafe { asm::wfe() }
}
