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
        kinfo!("GOP initialized");
    } else {
        kerror::kerror();
        kinfo!("Failed to initialize GOP");
    }
}

// For ueserspace
pub unsafe fn gop_reinit(bt: &BootServices) -> &mut GraphicsOutput {
    if let Ok(gop) = bt.locate_protocol::<GraphicsOutput>() {
        let gop = gop.expect("Couldn't initialize GOP");
        let gop = &mut *gop.get();
        return gop;
    } else {
        kerror::kerror();
        kinfo!("Couldn't reinitialize GOP");
        // TODO: Find a better way of doing this
        return gop_reinit(bt);
    }
}
