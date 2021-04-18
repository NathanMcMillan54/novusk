pub mod error;
pub mod init;
pub mod tests;
pub mod time;
pub mod vga;

extern "C" {
    pub fn kernel_init();
}