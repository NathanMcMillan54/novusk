use crate::drivers::device::arch::Arch;

#[repr(C)]
pub struct Device {
    pub cpu_brand: &'static str,
    pub cpu_arch: Arch,
    pub device_name: &'static str,
    pub device_company: &'static str,
}
