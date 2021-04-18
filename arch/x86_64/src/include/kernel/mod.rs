use crate::include::asm::{hlt};

pub unsafe fn die() -> ! {
    hlt();
    die()
}
