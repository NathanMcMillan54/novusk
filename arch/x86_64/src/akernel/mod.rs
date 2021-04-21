pub mod init;
pub mod syscalls;

extern "C" {
    pub(crate) fn kernel_init();
}