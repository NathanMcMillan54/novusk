global_asm!(include_str!("risc.S"));
global_asm!(include_str!("trap.S"));

extern "C" {
    fn wait_for_interrupt() -> !;
}

#[no_mangle]
pub unsafe extern "C" fn wfi() -> ! {
    wait_for_interrupt();
}

