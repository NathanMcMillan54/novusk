#![no_std]

#[macro_use] extern crate alloc;
#[macro_use] extern crate kinfo;
#[macro_use] extern crate printk;

use konfig::KONFIG;

pub(crate) mod syscall;
pub(crate) mod user;


pub fn setup_kernel() {
    syscall::check_sys_nums();
}

pub fn after_kernel_setup() {
    let mut configs = KONFIG.lock();
    let after = configs.get("KERNEL", "AFTER");

    if after == "OS" || after == "SERVER" {
        user::user_setup();
    } else if after == "Nothing" {

    } else {
        panic!("{} isn't a KERNEL_AFTER option, ending the kernel here (there wasn't much to be done anyway).", after);
    }
}
