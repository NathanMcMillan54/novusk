use r0::zero_bss;

extern "C" {
    pub static mut __bss_end: u64;
    pub static mut __bss_start: u64;
}

pub unsafe fn clear_bss_se() {
    zero_bss(&mut __bss_start, &mut __bss_end);
}
