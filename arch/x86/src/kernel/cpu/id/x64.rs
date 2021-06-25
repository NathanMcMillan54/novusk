use super::BRAND;
use kinfo::info::set_info;
use raw_cpuid::CpuId;
use crate::kernel::kernel::x86_printk;

unsafe fn unknown_cpu() {
    set_info("not ok");
    BRAND = BRAND;
}

unsafe fn its_amd() {
    BRAND = "AMD";
}

unsafe fn its_intel() {
    BRAND = "Intel";
}

pub unsafe fn get_cpuid() {
    let mut cpuid = CpuId::new();

    BRAND = "test";
    match cpuid.get_vendor_info() {
        Some(vi) =>
            if vi.as_string() == "AuthenticAMD" {
                its_amd();
            } else if vi.as_string() == "GenuineIntel" {
                its_intel();
            },
        None =>
        // ('_') <(Oh no!)
            unknown_cpu()
    }
}
