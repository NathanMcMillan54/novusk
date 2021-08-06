global_asm!(include_str!("boot64.S"));

use crate::aarch64_printk;
use crate::kernel::init::aarch64_init;
use core::fmt::{Result, Write};
use crate::kernel::debug::{DebugPrint, console};

#[no_mangle]
#[link_section = ".text._start_arguments"]
pub static BOOT_CORE_ID: u64 = 0;

extern "C" {
    pub static mut __bss_end: u64;
    pub static mut __bss_start: u64;
}

#[no_mangle]
pub unsafe extern "C" fn aarch64_boot_setup() -> ! {
    r0::zero_bss(&mut __bss_start, &mut __bss_end);

    //write_string("Starting kernel...\n");

    // TODO: Figure out why this crashes everything
    // aarch64_printk!("Starting kernel...\n");

    let mut dprint = DebugPrint;

    dprint.write_string("Starting kernel...\n");
    dprint.write_string("\n");

    aarch64_init();

    loop { asm!("wfe"); }
}
