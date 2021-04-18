global_asm!(include_str!("x86.S"));

extern "C" {
    fn clear_df();
    fn clear_if();
    fn halt();
}

pub unsafe fn cld() {
    clear_df();
}

pub unsafe fn cli() {
    clear_if();
}

pub unsafe fn hlt() {
    halt();
}
