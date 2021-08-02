use super::boot::{die, boot_init};
use crate::kernel::kernel::*;
use crate::kernel::x86_start::start_x86;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    boot_init();
    x86_printk!("Starting kernel...");

    start_x86();

    die()
}
