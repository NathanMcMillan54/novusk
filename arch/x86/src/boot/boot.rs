use uefi::Handle;
use uefi::table::{Boot, SystemTable};
use crate::include::asm::hlt;
use libefi::st;
use uefi::proto::console::text::Color;


pub unsafe fn die() -> ! {
    st().as_ref().stdout().set_color(Color::Red, Color::Black);
    printk!("\nKernel died");
    printk!("   Starting endless loop...");
    hlt()
}
