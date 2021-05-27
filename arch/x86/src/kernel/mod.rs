pub mod cpu;
pub mod dev;
pub mod early;
pub mod init;
pub mod io;
pub mod modules;
pub mod printk;
pub mod st;

pub static mut KERNEL_INFO: bool = true;
