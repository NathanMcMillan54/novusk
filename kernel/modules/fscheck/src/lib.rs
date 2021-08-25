#![no_std]

extern crate alloc;

pub mod config;

use kinfo::info::FS;

pub unsafe fn fscheck_init() {
    if FS == "None" || FS == "" {
        return;
    }

    let mut fs = config::get_fs();

    fscheck_end();
}

pub fn fscheck_end() {

}
