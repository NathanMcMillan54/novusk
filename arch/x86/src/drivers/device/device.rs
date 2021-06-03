use cpu::{arch::Arch, cpu::Cpu};

#[repr(C)]
pub struct Device {
    pub cpu_brand: Cpu,
    pub cpu_arch: Arch,
    pub device_name: &'static str,
    pub device_company: &'static str,
}
