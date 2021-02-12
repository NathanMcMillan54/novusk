// WFE
#[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
global_asm!(include_str!("wfe.S"));

extern "C" { fn asm_loop() -> !; }

pub unsafe fn wfe() -> ! {
    asm_loop()
}
