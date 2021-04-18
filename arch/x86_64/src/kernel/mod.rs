pub mod init;
pub mod vga;

extern "C" {
    pub fn kernel_init();
}