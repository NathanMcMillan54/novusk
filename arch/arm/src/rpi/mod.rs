use crate::mm::linker_mem;
use gpu::{DriverNames, info::set_gpu};

#[cfg(target_arch = "aarch64")]
#[no_mangle]
pub unsafe extern "C" fn aarch64_rpi_setup() {
    extern "C" {
        static __bss_start: u64;
        static __bss_end: u64;
    }

    linker_mem::clear_bss_se(__bss_start, __bss_end);

    set_gpu(DriverNames::RpiFb);
}
