use r0::zero_bss;

extern "C" {
    static mut __bss_start: u64;
    static mut __bss_end: u64;
}

#[no_mangle]
pub unsafe extern "C" fn zero_bss_se() {
    zero_bss(&mut __bss_start, &mut __bss_end);
}
