global_asm!(include_str!("boot64.S"));

use crate::kernel::init::aarch64_init;
use arm::rpi::aarch64_rpi_setup;

#[no_mangle]
#[link_section = ".text._start_arguments"]
pub static BOOT_CORE_ID: u64 = 0;

extern "C" {
    pub static mut __bss_end: u64;
    pub static mut __bss_start: u64;
}

#[no_mangle]
pub unsafe extern "C" fn aarch64_boot_setup() -> ! {
    aarch64_rpi_setup();

    aarch64_init();

    loop { asm!("wfe"); }
}
