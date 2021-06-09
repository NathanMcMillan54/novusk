use cpu::arch::Arch;
use cpu::cpu::Cpu;

#[cfg(feature = "default_machine")]
pub mod default;

#[cfg(feature = "uefi_rpi3")]
pub mod uefi_rpi3;

#[cfg(feature = "qemu_virt")]
pub mod qemu_virt;

pub struct Device {
    pub board: &'static str,
    pub manufacture: &'static str,
    pub cpu_arch: Arch,
    pub cpu_brand: Cpu,
    pub main_kernel: bool,
    pub arch_kernel: bool,
}

pub static mut DEVICE_INFO: Device = Device {
    board: "",
    manufacture: "",
    cpu_arch: Arch::Unknown,
    cpu_brand: Cpu::Unknown,
    main_kernel: false,
    arch_kernel: false
};

pub unsafe fn device_init(dev_info: Device) {
    DEVICE_INFO = dev_info;
}
