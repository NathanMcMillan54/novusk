// Read ``Documentation/x86_64/cpus.md`` for Novusk's requirements for running on an x86_64 CPU.
use raw_cpuid::CpuId;

extern "C" {
    fn check_cpuid();
}

pub unsafe fn validate_cpu() -> bool {
    // Checks if CPUID is available. If it isn't ``die()`` gets called because the CPU might not be
    // supported at all.
    check_cpuid();

    // Check the CPU brand
    /*let mut cpuid = CpuId::new();
    let brand = cpuid.get_processor_brand_string().unwrap();
    let name = brand.as_str();

    match name {
        "GenuineIntel" => {

        },
        "AuthenticAmd" => {

        },
        _ => { return false; }
    };*/

    return true;
}

core::arch::global_asm!(include_str!("cpu.S"));
