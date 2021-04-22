pub mod init;
pub mod modules;
pub mod tests;
pub mod syscalls;
pub mod time;

extern "C" {
    pub(crate) fn kernel_init();
}