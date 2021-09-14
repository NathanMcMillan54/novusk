use crate::define_syscall;

pub const MAJOR_VERSION: i32 = 3;
pub const MINOR_VERSION: i32 = 0;
pub const REALLY_MINOR_VERSION: i32 = 0;
pub const VERSION_NAME: &str = "beta";

// -----------------
// Version/sys_version
//
// This is used for getting the major version of Novusk
fn version(sys_arg: u8) -> u8 {
    return MAJOR_VERSION as u8;
}

define_syscall!(sys_version, version);
