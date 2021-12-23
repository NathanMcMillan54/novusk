use novusk_syscalls::{SysCallTable};

#[no_mangle]
pub static mut SYSCALL_TABLE: SysCallTable = SysCallTable::new();
