use crate::include::asm::cli;

pub unsafe fn keyboard_init() {
    cli();
    asm!("mov ah, 0x02");
    asm!("mov ax, 0x0305");
}
