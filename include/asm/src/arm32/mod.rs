use core::arch::asm;

pub mod ints;

pub unsafe fn wfi() {
    asm!("wfi");
}

pub unsafe fn wfe() {
    asm!("wfe");
}
