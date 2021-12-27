use novusk_syscalls::SysCallTable;

pub mod syscall;
pub mod sys_tbl;

extern "C" {
    pub(crate) static mut SYSCALL_TABLE: SysCallTable;
    pub(self) fn sys_read(sys_arg1: u8, sys_arg2: u8, sys_arg3: u8) -> u8;
    pub(self) fn sys_write(sys_arg1: u8, sys_arg2: u8, sys_arg3: u8) -> u8;
    pub(self) fn sys_write_init(sys_arg1: u8, sys_arg2: u8, sys_arg3: u8) -> u8;
}
