#![no_std]
#![feature(alloc_error_handler)]

#[cfg(feature = "libc")]
#[path = "lang.rs"]
pub mod lang;

pub extern crate novuskinc;

#[no_mangle]
pub unsafe extern "C" fn syscall(sys_num: u32, sys_arg1: u8, sys_arg2: u8, sys_arg3: u8) -> u8 {
    0
}

#[no_mangle]
pub unsafe extern "C" fn add_syscalls() -> i32 {
    1
}
