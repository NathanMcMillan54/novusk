use crate::drivers::device::{uefi_rpi3};

pub unsafe fn device_init() {
    #[cfg(feature = "uefi_rpi3")]
    uefi_rpi3::uefi_pi3_init();
}
