#![no_std]

#[macro_use] extern crate printk;
#[macro_use] extern crate novuskinc;

pub mod common;
pub mod ksys;

use common::*;
use ksys::*;

/// Handles all non architecture specific syscalls
#[no_mangle]
pub unsafe extern "C" fn do_syscall(sys_num: usize, sys_args: &[*const u8]) {
    match sys_num as usize {
        KSYS_WRITE => {
            ksys_write(sys_args[0] as u8, sys_args[1] as u8, sys_args[2] as u8);
        },
        _ => { printk!("{} is not an implemented syscall\n", sys_num); }
    }
}
