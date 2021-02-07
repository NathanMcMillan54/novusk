#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
global_asm!(include_str!("init.S"));
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
global_asm!(include_str!("kernel.S"));

mod main;
pub(crate) mod msg;

extern "C" {
    pub fn startKernel() -> !;
}
