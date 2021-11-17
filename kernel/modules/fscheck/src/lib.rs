#![no_std]

#[macro_use] extern crate alloc;
#[macro_use] extern crate novuskinc;

pub mod config;

use kinfo::info::FS;

pub fn fscheck_start() {
    let mut fs = config::get_fs();

    if fs == "None" || fs == "" {
        return;
    }
}

module_init!(fscheck_init, fscheck_start);

pub fn fscheck_finish() {

}

module_end!(fscheck_end, fscheck_finish);
