use novusk_syscalls::{SysCall, SysCallTable};
//use super::{*, sys_tbl::*};

extern "C" {
    // pub(crate) static mut SYSCALL_TABLE: SysCallTable;
    // pub(self) fn sys_read(sys_arg1: u8, sys_arg2: u8, sys_arg3: u8) -> u8;
    pub(self) fn sys_write(sys_arg1: u8, sys_arg2: u8, sys_arg3: u8) -> u8;
    pub(self) fn sys_write_init(sys_arg1: u8, sys_arg2: u8, sys_arg3: u8) -> u8;
    pub(self) fn sys_reboot(sys_arg1: u8, sys_arg2: u8, sys_arg3: u8) -> u8;
    pub(self) fn sys_shutdown(sys_arg1: u8, sys_arg2: u8, sys_arg3: u8) -> u8;
}

#[no_mangle]
pub static mut SYSCALL_TABLE: SysCallTable = SysCallTable::new();

pub const READ: u32 = 0;
pub const WRITE: u32 = 1;
pub const REBOOT: u32 = 20;
pub const MODULE: u32 = 21;
pub const SHUTDOWN: u32 = 22;
pub const VERSION: u32 = 30;
pub const UNAME: u32 = 31;
pub const KINFO: u32 = 32;
pub const WRITE_INIT: u32 = 35;


pub unsafe fn syscalls_init() {
    SYSCALL_TABLE.start_init();
    SYSCALL_TABLE.set_name("x86_64 Novusk System call Table");

    SYSCALL_TABLE.add_syscall(SysCall::new("sys_read", READ, sys_read));
    SYSCALL_TABLE.add_syscall(SysCall::new("sys_write", WRITE, sys_write));
    SYSCALL_TABLE.add_syscall(SysCall::new("write_init", WRITE_INIT, sys_write_init));
    SYSCALL_TABLE.add_syscall(SysCall::new("sys_shutdown", SHUTDOWN, sys_shutdown));
    SYSCALL_TABLE.add_syscall(SysCall::new("sys_reboot", REBOOT, sys_reboot));
}

#[no_mangle]
pub extern "C" fn sys_read(arg1: u8, arg2: u8, arg3: u8) -> u8 {
    0
}
