#![no_std]

extern crate kerror;
#[macro_use] extern crate kinfo;

extern crate uefi;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::prelude::BootServices;
use uefi::ResultExt;

// TODO: Get screen width an height
pub static mut SCREEN_HEIGHT: usize = 768;
pub static mut SCREEN_WIDTH: usize = 1024;

unsafe fn set_mode(gop: &mut GraphicsOutput) {
    let mode = gop
        .modes()
        .map(|mode| mode.expect("Warnings encountered while querying mode"))
        .find(|mode| {
            let info = mode.info();
            info.resolution() == (1024, 768)
        })
        .unwrap();

    gop.set_mode(&mode)
        .expect_success("Failed to set graphics mode");
}

pub unsafe fn gop_init(bt: &BootServices) {
    if let Ok(gop) = bt.locate_protocol::<GraphicsOutput>() {
        let gop = gop.expect("Couldn't initialize GOP");
        let gop = &mut *gop.get();
        set_mode(gop);
        kinfo!("GOP initialized");
    } else {
        kerror::kerror();
        kinfo!("Failed to initialize GOP");
    }
}

// For ueserspace
#[no_mangle]
pub unsafe extern "C" fn gop_reinit(bt: &BootServices) -> &'static mut GraphicsOutput {
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
