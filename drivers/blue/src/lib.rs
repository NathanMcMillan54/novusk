#![no_std]

use init::init::KERNEL;
use libost::traits::Setup;

pub(crate) mod setup;

pub struct Blue;

impl Blue {
    pub fn new() -> Self {
        return Blue;
    }
}

pub fn blue_init() {
    let mut blue = Blue::new();

    unsafe { KERNEL.lock().gpu_graphics().init(); }
    blue.init("Kernel is almost finished...\n");
}
