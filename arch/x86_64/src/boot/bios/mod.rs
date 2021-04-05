pub mod keyboard;
pub mod print;

use crate::boot::intcall;

pub unsafe fn set_bios_mode() {
    asm!(
    "mov ax, 0xec00",
    "mov bx, 2"
    );
    intcall(0x15, 0xec00, 0);
}
