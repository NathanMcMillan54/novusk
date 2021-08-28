#![no_std]

#[macro_use] extern crate alloc;
#[macro_use] extern crate novuskinc;

pub mod config;

use kinfo::info::FS;

pub fn fscheck_init() {
    let mut fs = config::get_fs();

    if fs == "None" || fs == "" {
        return;
    }
}

pub fn fscheck_end() {

}
