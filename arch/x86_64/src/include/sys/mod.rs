use novusk_syscalls::SysCallTable;

pub mod syscall;
pub mod sys_tbl;

extern "C" {
    pub(crate) static mut SYSCALL_TABLE: SysCallTable;
}
