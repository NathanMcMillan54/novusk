pub(crate) unsafe fn clear_bss(start: *mut u32, end: *mut u32) {
    r0::zero_bss(start, end);
}
