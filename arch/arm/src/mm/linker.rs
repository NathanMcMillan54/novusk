extern "C" {
    pub static __sbss: u64;
    pub static __ebss: u64;
}

pub unsafe fn clear_bss_sections() {
    r0::zero_bss(__sbss as *mut u64, __ebss as *mut u64);
}
