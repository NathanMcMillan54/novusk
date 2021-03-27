global_asm!(include_str!("x86.S"));

extern "C" {
    fn halt() -> !;
    fn clear();
}

pub unsafe fn hlt() -> ! {
    loop { halt() }
}

pub unsafe fn cli() {
    clear();
}