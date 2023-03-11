use core::arch::{asm};

pub mod main;

#[no_mangle]
pub unsafe extern "C" fn die() -> ! {
    loop { asm!("hlt"); }
}
