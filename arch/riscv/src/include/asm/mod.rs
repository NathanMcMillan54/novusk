global_asm!(include_str!("risc.S"));

#[cfg(target_arch = "riscv64")]
global_asm!(include_str!("trap.S"));

extern "C" {
    fn wait_for_interrupt() -> !;
}

#[no_mangle]
pub unsafe extern "C" fn wfi() -> ! {
    wait_for_interrupt();
}

