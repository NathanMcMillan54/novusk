#![no_std]
#![allow(warnings)]

#[macro_use] extern crate novuskinc;

use novuskinc::core::prelude::*;

pub mod vga;

#[no_mangle]
pub static mut FB: FrameBuffer = FrameBuffer::empty();

unsafe fn vgag_init() {
    FB.set("VGA FrameBuffer", (0, 0), 0x00000 as *mut u8);
}

module_init!(core_display_init, vgag_init);

fn vgag_end() {

}

module_end!(core_display_end, vgag_end);

/*#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {  }
}*/