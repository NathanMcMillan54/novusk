use crate::drivers::vga::_vga_print;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    _vga_print(format_args!("{}", "Starting kernel...\n"));

    loop { asm!("hlt"); }
}
