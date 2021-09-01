global_asm!(include_str!("arm.S"));

extern "C" {
    fn wait_for_event() -> !;
    fn no_op();
}

#[no_mangle]
pub unsafe extern "C" fn wfe() -> ! {
    wait_for_event()
}

#[no_mangle]
pub unsafe extern "C" fn nop() {
    no_op();
}
