use super::{cpu, kernel_init};
use crate::drivers::{early_hardware_init};
use crate::include::{kernel::die};

#[no_mangle]
pub unsafe extern "C" fn main() {
    /* if !cpu::validate_cpu() {
        die()
    } else {
        cpu::ARCHITECTURE = "x86_64";
    } */

    /* early_hardware_init();

    kernel_init(); */
}
