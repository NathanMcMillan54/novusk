use crate::include::asm::{cld, cli, hlt};

pub unsafe fn clear_all() {
    cld();
    cli();
}

pub unsafe fn die() -> ! {
    clear_all();
    loop { hlt(); }
}
