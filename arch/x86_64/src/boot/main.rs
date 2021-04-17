use super::{cpu, kernel_init};
use crate::drivers;
use crate::include::{kernel::die};
use crate::kernel::init::init;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    if !cpu::validate_cpu() {
        die();
    }

    drivers::early_drivers_init();

    init();
    kernel_init();
    loop { asm!("hlt"); }
}
