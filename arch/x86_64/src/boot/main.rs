use super::{bios::{keyboard::keyboard_init, set_bios_mode}, cpu::validate_cpu, intcall, kernel_init, kernel_main};
use crate::drivers::{text::init::text_init};
use crate::include::{asm::cli, kernel::die};
use crate::kernel::{init::init, time::time_init};


unsafe fn protected_mode() {
    asm!(
        "mov ax, 0x13",
        "int 0x10"
    );
}

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    cli();

    if !validate_cpu() {
        die();
    }

    set_bios_mode();
    keyboard_init();

    text_init();
    protected_mode();
    init();
}
