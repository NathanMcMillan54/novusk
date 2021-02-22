use crate::ARCH;
use crate::boot::{msg::boot_msg, startKernel};
use super::super::include;
use crate::kernel::{cmdline, vga_buffer::{Buffer, Color, Color::*, ColorCode, Writer}, time::time::time_init};
use crate::x86lib::print::x86_print;
use crate::sleep;

pub unsafe fn x86_init() -> ! {
    x86_print(format_args!("Starting kernel on ARCH={}...\n", ARCH));
    time_init();
    e_kinfo!("Kernel clock initialized\n");
    startKernel()
}

#[no_mangle]
pub extern "C" fn x86_end_kernel() -> ! {
    unsafe { include::asm::hlt() }
}
