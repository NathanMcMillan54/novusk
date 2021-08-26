extern "C" {
    pub fn syscall(sys_num: i32, sys_arg: u8);
    pub fn sys_write(write: u8);
}

#[cfg(target_arch = "x86_64")]
pub mod x86_64 {
    pub const READ: i32 = 0;
    pub const WRITE: i32 = 1;
}

#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

#[cfg(target_arch = "aarch64")]
pub mod aarch64 {
    pub const WRITE: i32 = 4;
}

#[cfg(target_arch = "aarch64")]
pub use aarch64::*;
