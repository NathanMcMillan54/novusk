use core::fmt::Write;
use crate::early_printk;
use crate::kernel::video_vga::EarlyVga;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    early_printk!("Starting kernel...\n");



    panic!("x86_64 kernel ended");
}
