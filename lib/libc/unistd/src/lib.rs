#![no_std]

#[macro_use] extern crate cfg_if;

#[cfg(target_arch = "x86_64")]
#[path = "../../../../arch/x86_64/src/include/sys_tbl.rs"]
pub mod table;

#[cfg(target_arch = "aarch64")]
#[path = "../../../../arch/aarch64/src/include/asm/systbl.rs"]
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

use novuskinc::kernel::syscalls::*;

#[no_mangle]
pub unsafe extern "C" fn syscall(sys_num: i32, sys_arg: u8) -> u8 {
    cfg_if! {
        if #[cfg(any(target_arch = "aarch64", target_arch = "arm", target_arch = "x86_64"))] {
            match sys_num {
                VERSION => return sys_version(sys_arg),
                // WRITE => sys_write(sys_arg),
                READ => return sys_read(sys_arg),
                REBOOT => sys_reboot(sys_arg),
                _ => 0 + 0,
            };
        }
    }


    return 0;
}
