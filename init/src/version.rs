pub const MAJOR_VERSION: i32 = 3;
pub const MINOR_VERSION: i32 = 0;
pub const REALLY_MINOR_VERSION: i32 = 0;
pub const VERSION_NAME: &str = "beta";

pub fn kernel_version(sys_arg: u8) -> u8 {
    return MAJOR_VERSION as u8;
}

// define_syscall!(sys_kernel_version, kernel_version);
