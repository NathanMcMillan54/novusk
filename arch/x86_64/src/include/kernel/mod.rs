use crate::akernel::time::current_time;
use crate::include::asm::{hlt};

#[no_mangle]
pub unsafe extern "C" fn die() -> ! {
    panick!("\nKernel died at: [ {} ]\n", current_time());
    loop { hlt(); }
}
