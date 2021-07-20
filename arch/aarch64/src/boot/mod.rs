global_asm!(include_str!("boot64.S"));

use crate::aarch64_printk;
use arm::mm::linker_mem::{clear_bss_se};
use core::fmt::{Result, Write};

extern "C" {
    pub static mut __bss_end: u64;
    pub static mut __bss_start: u64;
}

fn write_string(s: &str) {
    for c in s.chars() {
        unsafe {
            core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8);
        }
    }
}

struct DebugPrint;

impl Write for DebugPrint {
    fn write_str(&mut self, s: &str) -> Result {
        write_string(s);
        Ok(())
    }
}

#[no_mangle]
pub unsafe extern "C" fn aarch64_boot_setup() -> ! {
    clear_bss_se(__bss_start, __bss_end);

    write_string("Starting kernel...\n");

    // TODO: Figure out why this crashes everything
    // aarch64_printk!("Starting kernel...\n");


    loop { asm!("wfe"); }
}
