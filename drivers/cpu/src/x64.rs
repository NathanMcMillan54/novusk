use super::cpu::Cpu;
use raw_cpuid::CpuId;

fn check_cpu_brand() -> Cpu {
    let cpuid = CpuId::new();
    match cpuid.get_vendor_info() {
        Some(vi) =>
            if vi.as_string() == "AuthenticAMD" {
                return Cpu::AMD;
            } else if vi.as_string() == "GenuineIntel" {
                return Cpu::Intel;
            } else {
                return Cpu::Unknown;
            }
        None => return Cpu::Unknown
    }
}

pub fn look_for_amd() -> bool {
    let cpuid = CpuId::new();
    match cpuid.get_vendor_info() {
        Some(vi) =>
            if vi.as_string() == "AuthenticAMD" {
                return true;
            } else {
                return false;
            },
        None => return false
    }
}

pub fn look_for_intel() -> bool {
    let cpuid = CpuId::new();
    match cpuid.get_vendor_info() {
        Some(vi) =>
            if vi.as_string() == "GenuineIntel" {
                return true;
            } else {
                return false;
            },
        None => return false
    }
}

pub fn cpu_brand() -> Cpu {
    // Double check CPU brand
    if look_for_amd() == true {
        return Cpu::AMD;
    } else if look_for_intel() == true {
        return Cpu::Intel;
    } else {
        // Check again because AMD doesn't show up sometimes
        if CpuId::new().get_vendor_info().unwrap().as_string() == "AuthenticAMD" {
            return Cpu::AMD;
        } else {
            return Cpu::Unknown;
        }
    }
}
