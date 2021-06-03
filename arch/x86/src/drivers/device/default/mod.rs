use crate::drivers::device::device::Device;
use crate::kernel::dev::setup::set_device;
use crate::kernel::cpu::BRAND;
use cpu::{arch::Arch, cpu::Cpu};

fn get_arch() -> Arch {
    #[cfg(target_arch = "x86_64")]
    return Arch::X86_64;

    #[cfg(target_arch = "x86")]
    return Arch::X86;
}

pub unsafe fn default_init() {
    let arch = get_arch();
    #[cfg(target_arch = "x86_64")]
    set_device(
        Device {
            cpu_brand: BRAND,
            cpu_arch: arch,
            device_name: "Default",
            device_company: "None"
        }
    );

    #[cfg(target_arch = "x86")]
    set_device(
        Device {
            // The CPU is likely Intel
            cpu_brand: Cpu::Intel,
            cpu_arch: arch,
            device_name: "Default",
            device_company: "None"
        }
    );
}
