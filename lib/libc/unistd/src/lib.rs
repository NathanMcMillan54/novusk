#![no_std]

#[macro_use] extern crate cfg_if;

#[cfg(target_arch = "x86_64")]
#[path = "../../../../arch/x86_64/src/include/sys/sys_tbl.rs"]
pub mod table;

#[cfg(target_arch = "aarch64")]
#[path = "../../../../arch/aarch64/src/include/sys/systbl.rs"]
pub mod table;

#[cfg(target_arch = "arm")]
#[path = "../../../../arch/arm32/src/include/asm/sys_tbl.rs"]
pub mod table;

#[cfg(target_arch = "xtensa")]
#[path = "../../../../arch/xtensa/src/include/tbl.rs"]
pub mod table;

#[cfg(target_arch = "riscv32")]
#[path = "../../../../arch/riscv/src/include/syscall_table.rs"]
pub mod table;

use table::*;
pub use table::*;

use novuskinc::kernel::syscalls::table::SYSCALL_TABLE;

#[no_mangle]
pub unsafe extern "C" fn syscall(sys_num: u32, sys_arg1: u8, sys_arg2: u8, sys_arg3: u8) -> u8 {
    let ret = SYSCALL_TABLE.make_call(sys_num, sys_arg1, sys_arg2, sys_arg3);

    return ret;
}
