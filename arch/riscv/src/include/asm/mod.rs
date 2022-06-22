use core::arch::global_asm;

global_asm!(include_str!("risc.S"));

extern "C" {
    fn wait_for_int();
}

pub unsafe fn wfi() -> ! {
    loop { wait_for_int(); }
}
