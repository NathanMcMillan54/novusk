pub mod cpu;
mod main;
pub mod power;

extern "C" {
    pub(crate) fn kernel_init();
}