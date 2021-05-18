#![no_std]

extern crate kerror;
#[macro_use] extern crate kinfo;

extern crate uefi;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::prelude::BootServices;

pub unsafe fn gop_init(bt: &BootServices) {
    if let Ok(gop) = bt.locate_protocol::<GraphicsOutput>() {
        let gop = gop.expect("Couldn't initialize GOP");
        let gop = &mut *gop.get();
        kinfo!("GOP init");
    } else {
        kerror::kerror();
        kinfo!("Failed to initialize GOP");
    }
}
