use crate::drivers::device::device::Device;
use crate::drivers::device::{android_box, default};
use cpu::cpu::Cpu;
use cpu::arch::Arch;

pub mod setup;

pub static mut DEVICE_INFO: Device = Device {
    cpu_brand: Cpu::Unknown,
    cpu_arch: Arch::X86_64,
    device_name: "",
    device_company: ""
};

pub unsafe fn device_init() {
    #[cfg(feature = "default_machine")]
    default::default_init();

    #[cfg(feature = "android_box")]
    android_box::android_box_init();
}
