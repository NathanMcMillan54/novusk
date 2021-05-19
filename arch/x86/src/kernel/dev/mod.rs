use crate::drivers::device::{arch::Arch, device::Device};
use crate::drivers::device::{default};

pub mod setup;

pub static mut DEVICE_INFO: Device = Device {
    cpu_brand: "",
    cpu_arch: Arch::x86_64,
    device_name: "",
    device_company: ""
};

pub unsafe fn device_init() {
    #[cfg(feature = "default_machine")]
    default::default_init();
}
