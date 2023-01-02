use core::arch::asm;

pub unsafe fn el() -> u64 {
    let mut ret = 0;
    asm!("mrs {}, CurrentEl", out(reg) ret);

    return ret >> 2;
}
