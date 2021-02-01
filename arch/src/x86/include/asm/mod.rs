// HLT
global_asm!(include_str!("hlt.S"));
extern "C" {
    fn asm_loop() -> !;
}

pub unsafe fn hlt() ->! {
    asm_loop()
}

// BIOS Shutdown
global_asm!(include_str!("shutdown.S"));
extern "C" {
    fn asm_shutdown() -> !;
}

pub unsafe fn shutdown() -> ! {
    asm_shutdown();
}