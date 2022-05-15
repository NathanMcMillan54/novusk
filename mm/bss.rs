use core::mem::zeroed;
use core::ptr::write_volatile;
use core::sync::atomic::{compiler_fence, Ordering};

pub unsafe fn zero_bss(mut sbss: *mut u32, mut ebss: *mut u32) {
    while sbss < ebss {
        write_volatile(sbss, zeroed());
        sbss = sbss.offset(1);
    }

    compiler_fence(Ordering::SeqCst);
}

pub fn is_zeroed(ebss: u32) -> bool {
    if ebss != 0 {
        return false;
    } else { return true; }
}
