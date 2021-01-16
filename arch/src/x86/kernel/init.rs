use crate::ARCH;
use crate::x86::kernel::vga_buffer::{Buffer, Color, Color::*, ColorCode, Writer};
use crate::x86::include::time;
use crate::x86::kernel::cmdline;
use crate::x86::lib::print::x86_print;
use crate::x86::boot::startKernel;

fn boot_msg(msg: &str, pos: i32, color: Color) {
    let mut writer = Writer {
        column_position: pos as usize,
        color_code: ColorCode::new(color, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };
    writer.write_string(msg);
}

pub unsafe fn x86_init() -> ! {
    boot_msg("Starting", 0, White);
    boot_msg(" Novusk...\n", 8, Cyan);
    boot_msg("v1.0.0 New Kernel", 0, Cyan);
    time::sleep(1);
    boot_msg("\n\nSetting up cmdline...", 0, White);
    time::sleep(1);
    cmdline::setup_cmdline();
    x86_print(format_args!("Starting kernel on ARCH={}...\n", ARCH));
    startKernel()
}
