global_asm!(include_str!("init.S"));
global_asm!(include_str!("kernel.S"));

mod main;

extern "C" {
    pub fn startKernel() -> !;
}
