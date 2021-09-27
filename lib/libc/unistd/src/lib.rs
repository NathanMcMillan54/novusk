#![no_std]

#[macro_use] extern crate cfg_if;

#[cfg(target_arch = "x86_64")]
#[path = "../../../../arch/x86_64/src/include/sys_tbl.rs"]
pub mod table;

#[cfg(target_arch = "aarch64")]
#[path = "../../../../arch/aarch64/src/kernel/sys.rs"]
pub mod table;

#[cfg(target_arch = "arm")]
#[path = "../../../../arch/arm/src/include/sys_tbl.rs"]
pub mod table;

#[cfg(target_arch = "xtensa")]
#[path = "../../../../arch/xtensa/src/include/tbl.rs"]
pub mod table;

pub use table::*;
use table::*;

use novuskinc::kernel::syscalls::*;

pub unsafe fn syscall(sys_num: i32, sys_arg: u8) -> u8 {
    cfg_if! {
        if #[cfg(any(target_arch = "aarch64", target_arch = "arm", target_arch = "x86_64"))] {
            match sys_num {
                VERSION => return sys_version(sys_arg),
                WRITE => sys_write(sys_arg),
                READ => return sys_read(sys_arg),
                REBOOT => sys_reboot(sys_arg),
                _ => 0 + 0,
            };
        }
    }


    return 0;
}
