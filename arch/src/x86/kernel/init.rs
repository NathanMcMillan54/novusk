use crate::ARCH;
use crate::x86::boot::{msg::boot_msg, startKernel};
use crate::x86::kernel::{cmdline, vga_buffer::{Buffer, Color, Color::*, ColorCode, Writer}};
use crate::x86::lib::print::x86_print;
use crate::sleep;

pub unsafe fn x86_init() -> ! {
    sleep(1);
    boot_msg("\nSetting up cmdline...", 0, White);
    sleep(1);
    cmdline::setup_cmdline();
    x86_print(format_args!("Starting kernel on ARCH={}...\n", ARCH));
    e_kinfo!("Kernel clock initialized\n");
    startKernel()
}
