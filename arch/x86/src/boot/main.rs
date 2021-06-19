use super::boot::{die, boot_init};
use crate::drivers::vga::_vga_print;
use crate::kernel::x86_main::x86_main;
use crate::boot::boot::BOOT;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    _vga_print(format_args!("{}", "Starting kernel...\n"));
    boot_init();
    _vga_print(format_args!("{}{}", "x86 boot initialized\nBoot method: ", BOOT));

    x86_main();
}
