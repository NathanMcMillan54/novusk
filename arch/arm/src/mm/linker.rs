extern "C" {
    pub static __bss_start: u64;
    pub static __bss_end: u64;
}

pub unsafe fn clear_bss_sections() {
    r0::zero_bss(__bss_start as *mut u64, __bss_end as *mut u64);
}
