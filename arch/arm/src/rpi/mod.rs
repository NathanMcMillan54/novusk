use gpu::{DriverNames, info::set_gpu};

#[cfg(target_arch = "aarch64")]
#[no_mangle]
pub unsafe extern "C" fn aarch64_rpi_setup() {
    set_gpu(DriverNames::RpiFb);
}
