#![no_std]

#[cfg(target_arch = "x86_64")]
#[path = "../../../../arch/x86_64/src/include/sys_tbl.rs"]
pub mod table;
pub use table as _;
use table::*;

use novuskinc::syscalls::*;

pub unsafe fn syscall(sys_num: i32, sys_arg: u8) -> u8 {
    match sys_num {
        VERSION => return sys_version(sys_arg),
        _ => return 0,
    };

    return 0;
}
