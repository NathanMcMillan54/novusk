use crate::include::asm::{cli, hlt};

fn stack_overflow() -> ! {
    stack_overflow()
}

pub unsafe fn die() {
    #[cfg(any(target_arch = "x86_64"))]
    loop {
        cli();
        hlt();
        stack_overflow();
    }

    #[cfg(not(target_arch = "x86_64"))]
    loop {
        stack_overflow();
    }
}