use super::vga::HEIGHT;
use crate::kernel::kernel::*;

pub unsafe fn grub_init() {
    for i in 0..HEIGHT {
        x86_printk!(" ");
    }
}
