global_asm!(include_str!("init.S"));
global_asm!(include_str!("kernel.S"));

mod main;
pub(crate) mod msg;

extern "C" {
    pub fn startKernel() -> !;
}
