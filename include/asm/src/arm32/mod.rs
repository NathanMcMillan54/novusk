use core::arch::asm;

pub mod ints;

#[no_mangle]
pub unsafe extern "C" fn wfi() {
    asm!("wfi");
}
