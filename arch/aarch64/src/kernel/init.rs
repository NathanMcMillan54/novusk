use arm::include::asm;
use crate::boot::msg::boot_msg;
use super::{cmdline, early_info::*, time::time};
use crate::sleep;

pub unsafe fn aarch64_init() -> ! {
    time::time_init();
    _e_kinfo(format_args!("{}", "Time initialized\n"));
    aarch64_end_kernel()
}

#[no_mangle]
pub extern "C" fn aarch64_end_kernel() -> ! {
    unsafe { asm::wfe() }
}
