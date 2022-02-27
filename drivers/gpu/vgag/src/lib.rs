#![no_std]
#![allow(warnings)]

#[macro_use] extern crate novuskinc;

use novuskinc::core::prelude::*;

pub mod display;
pub mod vga;

use crate::vga::VgaG;

#[cfg(not(feature = "no_panic"))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {  }
}

#[no_mangle]
pub static mut FB: FrameBuffer = FrameBuffer::empty();

unsafe fn vgag_init() {

}

module_init!(core_display_init, vgag_init);

fn vgag_end() {

}

module_end!(core_display_end, vgag_end);
