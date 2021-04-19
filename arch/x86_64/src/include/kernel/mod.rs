use crate::akernel::time::current_time;
use crate::include::asm::{hlt};

pub unsafe fn die() -> ! {
    panick!("\nKernel died at: [ {} ]\n", current_time());
    loop { hlt(); }
}
