pub mod init;
pub mod modules;
pub mod tests;
pub mod syscalls;
pub mod thread;
pub mod time;

extern "C" {
    pub(crate) fn kernel_init();
    pub(crate) fn kernel_main() -> !;
}