global_asm!(include_str!("arm.S"));

extern "C" {
    fn wait() -> !;
}

pub unsafe fn wfe() -> ! {
    loop { wait(); }
}