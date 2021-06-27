use super::boot::{die, boot_init};
use crate::kernel::kernel::*;
use crate::kernel::x86_main::x86_main;
use crate::boot::boot::BOOT;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    boot_init();
    x86_printk!("Starting kernel...");

    x86_main();

    die()
}
