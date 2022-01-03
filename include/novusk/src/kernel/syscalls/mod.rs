#[cfg(target_arch = "x86_64")]
#[path = "../../../../../arch/x86_64/src/include/sys/syscall.rs"]
pub mod arch_syscalls;

#[cfg(target_arch = "xtensa")]
#[path = "../../../../../arch/xtensa/src/include/syscalls.rs"]
pub mod arch_syscalls;

#[cfg(target_arch = "aarch64")]
#[path = "../../../../../arch/aarch64/src/include/sys/setup.rs"]
pub mod arch_syscalls;

#[cfg(target_arch = "arm")]
#[path = "../../../../../arch/arm32/src/include/syscalls.rs"]
pub mod arch_syscalls;

#[cfg(target_arch = "riscv32")]
#[path = "../../../../../arch/riscv/src/include/syscalls.rs"]
pub mod arch_syscalls;

pub mod table;

use novusk_syscalls::SysCall;
use table::{DEFAULT_NAME, SYSCALL_TABLE};

use printk::printk;

extern "C" {
    pub fn sys_version(sys_arg1: u8, sys_arg2: u8, sys_arg3: u8) -> u8;
}

pub unsafe fn add_kernel_syscalls() {
    let syscalls = SYSCALL_TABLE.syscalls.as_ref().unwrap();

    let mut version_sysnum = 3;

    for sys_num in 0..syscalls.len() {
        if syscalls[sys_num].number == version_sysnum {
            version_sysnum += 1;
        }
    }

    SYSCALL_TABLE.add_syscall(SysCall::new("sys_version", version_sysnum, sys_version));
}

pub unsafe fn debug_syscall_table() {
    let (tbl_name, tbl_size) = SYSCALL_TABLE.get_table_info();

    for num in 0..tbl_size {
        let (sys_name, sys_num, sys_fun) = SYSCALL_TABLE.get_call_info(num as u32);

        printk!("[ {} ][ {} ]\n", sys_name, sys_num);
    }
}
