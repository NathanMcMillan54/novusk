pub mod init;
pub mod vga_80x25;
pub mod vga_320x200;

use core::fmt::Arguments;

pub static mut VGA_ADDRESS: usize = 0xb8000;
pub static mut VGA_ADDRESS_STR: &str = "0xb8000";
pub static mut HEIGHT: usize = 25;
pub static mut WIDTH: usize = 80;

pub unsafe fn _vga_print(fmt: Arguments) {
    if VGA_ADDRESS == 0xb8000 {
        vga_80x25::_print(fmt);
    } else if VGA_ADDRESS == 0xa0000 {
        vga_320x200::_print(fmt);
    }
}
