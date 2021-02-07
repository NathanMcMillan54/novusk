// HLT
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
global_asm!(include_str!("hlt.S"));
extern "C" {
    fn asm_loop() -> !;
}

pub unsafe fn hlt() -> ! {
    asm_loop()
}
