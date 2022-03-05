#![no_std]
#![allow(warnings)]

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate novuskinc;
#[macro_use] extern crate printk;

pub mod display;
pub mod vga;

use libcolor::{Color16, ColorCode};
use novuskinc::core::prelude::*;
use vga::vga_80x25::{Vga80x25Buffer, Vga80x25};
use vga::{VgaG, VgaMode};

#[cfg(not(feature = "no_panic"))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {  }
}

#[no_mangle]
pub static mut FB: FrameBuffer = FrameBuffer::empty();

unsafe fn vgag_init() {
    FB.set("VGA FrameBuffer",
           (80, 25),
           0xb8000 as *mut u8,
    &VgaG as &dyn FrameBufferGraphics);
}

module_init!(core_display_init, vgag_init);

fn vgag_end() {

}

module_end!(core_display_end, vgag_end);
