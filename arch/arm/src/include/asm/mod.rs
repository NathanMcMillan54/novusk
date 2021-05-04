global_asm!(include_str!("arm.S"));
extern "C" {
    fn halt() -> !;
}

pub unsafe fn wfe() -> ! {
    halt();
    wfe()
}
