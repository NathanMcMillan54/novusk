pub mod error;
pub mod init;
pub mod print_fmt;
pub mod tests;
pub mod time;
pub mod vga;

pub static mut KERNEL: bool = true;

extern "C" {
    pub fn kernel_init();
}