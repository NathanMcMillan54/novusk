global_asm!(include_str!("boot64.S"));

use arm::mm::linker_mem::{clear_bss_se};
use crate::kernel::test;

extern "C" {
    pub static mut __bss_end: u64;
    pub static mut __bss_start: u64;
}

#[no_mangle]
pub unsafe extern "C" fn aarch64_boot_setup() -> ! {
    clear_bss_se(__bss_start, __bss_end);

    loop { asm!("wfe"); }
}
