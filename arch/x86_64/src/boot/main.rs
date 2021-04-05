use super::{BiosRegs, cpu::validate_cpu, kernel_init, kernel_main};
use crate::include::{asm::cli, kernel::die};
use crate::kernel::{time::time_init};


unsafe fn protected_mode() {
    asm!(
        "mov ax, 0x13",
        "int 0x10"
    );
}

unsafe fn set_bios_mode() {
    asm!(
        "mov ax, 0xec00",
        "mov bx, 2"
    );
}

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    cli();


    if !validate_cpu() {
        die();
    }

    set_bios_mode();

    kernel_init();
    protected_mode();
    kernel_main()
}
