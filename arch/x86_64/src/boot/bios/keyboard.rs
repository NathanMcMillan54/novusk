use crate::boot::intcall;
use crate::include::{asm::cli};

pub unsafe fn keyboard_init() {
    cli();
    asm!("mov ah, 0x02");
    intcall(0x16, 0x02, 0);

    asm!("mov ax, 0x0305");
    intcall(0x16, 0x0305, 0);
}

pub unsafe fn bios_input() -> u8 {
    // TODO: Make BIOS input work
    0
}
