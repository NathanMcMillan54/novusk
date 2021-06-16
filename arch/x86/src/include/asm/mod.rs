global_asm!(include_str!("x86.S"));

extern "C" {
    pub(crate) fn clear();
    pub(crate) fn halt() -> !;
}

pub unsafe fn cli() {
    clear();
}

pub unsafe fn hlt() -> ! {
    loop { halt(); }
}
