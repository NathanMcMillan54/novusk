// HLT
global_asm!(include_str!("hlt.S"));
extern "C" {
    fn asm_loop() -> !;
}

pub unsafe fn hlt() ->! {
    asm_loop()
}
