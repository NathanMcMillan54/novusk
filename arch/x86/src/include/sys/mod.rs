global_asm!(include_str!("unistd.S"));

pub mod syscall;
pub mod syscalls;
pub mod sys_num;
