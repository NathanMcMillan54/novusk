use super::{cpu, kernel_init};
use crate::include::{kernel::die};

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    if !cpu::validate_cpu() {
        die()
    } else {
        #[cfg(target_arch = "x86_64")]
        cpu::ARCHITECTURE = "x86_64";

        #[cfg(target_arch = "x86")]
        cpu::ARCHITECTURE = "x86";
    }

    kernel_init();
    die()
}
