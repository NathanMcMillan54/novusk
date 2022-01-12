use novusk_syscalls::SysCall;
use crate::define_syscall;
use crate::kernel::syscalls::arch_syscalls::SYSCALL_TABLE;

pub const MAJOR_VERSION: i32 = 3;
pub const MINOR_VERSION: i32 = 0;
pub const REALLY_MINOR_VERSION: i32 = 0;
pub const VERSION_NAME: &str = "";

// -----------------
// Version/sys_version
//
// This is used for getting all version numbers of Novusk
fn version(sys_arg1: u8, sys_arg2: u8, sys_arg3: u8) -> u8 {
    if sys_arg1 == 0 {
        return MAJOR_VERSION as u8;
    } else if sys_arg1 == 1 {
        return MINOR_VERSION as u8;
    } else if sys_arg1 == 2 {
        return REALLY_MINOR_VERSION as u8;
    } else { unsafe { return *VERSION_NAME.as_ptr(); } }

    return MAJOR_VERSION as u8;
}

define_syscall!(sys_version, version);

pub unsafe fn version_init() {
    SYSCALL_TABLE.add_syscall(SysCall::new("sys_version", 30, sys_version));
}
