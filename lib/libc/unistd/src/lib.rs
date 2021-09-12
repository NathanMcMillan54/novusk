#![no_std]

#[cfg(target_arch = "x86_64")]
#[path = "../../../../arch/x86_64/src/include/sys_tbl.rs"]
pub mod table;
pub use table as _;
use table::*;

fn write() {

}

pub unsafe fn sys_call(sys_num: i32, sys_arg: u8) -> u8 {
    match sys_num {
        WRITE => write(),
        _ => return 0,
    }

    return 0;
}
