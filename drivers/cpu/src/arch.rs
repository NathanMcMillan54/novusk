#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Arch {
    Aarch64,
    X86_64,
    X86,
    Unknown,
}

pub fn cpu_arch() -> Arch {
    #[cfg(target_arch = "aarch64")]
    return Arch::Aarch64;

    #[cfg(target_arch = "x86_64")]
    return Arch::X86_64;

    #[cfg(target_arch = "x86")]
    return Arch::X86;
}
