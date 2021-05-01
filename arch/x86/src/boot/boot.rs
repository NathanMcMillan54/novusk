use uefi::Handle;
use uefi::table::{Boot, SystemTable};
use crate::include::asm::hlt;

pub struct UefiBootArgs {
    pub(crate) arg1: Handle,
    pub(crate) arg2: SystemTable<Boot>,
}

pub unsafe fn die() -> ! {
    hlt()
}
