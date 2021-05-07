use crate::boot::dev::setup::set_device;
use crate::drivers::device::arch::Arch;
use crate::drivers::device::device::Device;

fn get_arch() -> Arch {
    #[cfg(target_arch = "x86_64")]
    return Arch::x86_64;

    #[cfg(target_arch = "x86")]
    return Arch::i686;
}

pub unsafe fn default_init() {
    let arch = get_arch();
    set_device(
        Device {
            cpu_brand: "Unknown",
            cpu_arch: arch,
            device_name: "Default",
            device_company: "None"
        }
    );
}