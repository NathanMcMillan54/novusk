use super::cpu::Cpu;

// Most x86 CPUs are Intel
pub fn cpu_brand() -> Cpu {
    return Cpu::Intel;
}

