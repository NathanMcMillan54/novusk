global_asm!(include_str!("x86.S"));
extern "C" {
    fn halt() -> !;
}

pub unsafe fn hlt() -> ! {
    loop { halt(); }
}
