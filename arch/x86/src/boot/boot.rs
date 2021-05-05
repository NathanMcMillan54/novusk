use uefi::Handle;
use uefi::table::{Boot, SystemTable};
use crate::include::asm::hlt;


pub unsafe fn die() -> ! {
    hlt()
}
