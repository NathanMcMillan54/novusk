global_asm!(include_str!("arm.S"));

extern "C" {
    fn wait_fe() -> !;
}

pub unsafe fn wfe() -> ! {
    loop { wait_fe(); }
}
