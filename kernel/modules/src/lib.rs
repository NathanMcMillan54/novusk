#![no_std]


extern crate include;
use include::novusk::init;

#[macro_use]
extern crate kernel;

mod m1;
use m1::m1::{m1_exit, m1_init};

pub unsafe fn modules_init() {
    init::module_init(m1_init(), m1::m1::MODULE, m1::m1::AUTHOR);
    init::module_exit(m1_exit());
    info!("Modules init");
}
