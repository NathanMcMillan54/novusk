use super::cpu::Cpu;
use raw_cpuid::CpuId;

pub fn cpu_brand() -> Cpu {
    let cpuid = CpuId::new();
    match cpuid.get_vendor_info() {
        Some(vi) =>
            if vi.as_string() == "AuthenticAmd" {
                return Cpu::AMD;
            } else if vi.as_string() == "GenuineIntel" {
                return Cpu::Intel;
            } else {
                return Cpu::Unknown;
            }
        None => return Cpu::Unknown
    }
}
