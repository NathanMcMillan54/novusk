use crate::kernel::kernel::x86_printk;
use kinfo::info::set_info;
use raw_cpuid::CpuId;

pub static mut BRAND: &'static str = "Unknown";

unsafe fn unknown_cpu() {
    set_info("not ok");
    BRAND = BRAND;
}

pub unsafe fn get_cpuid() {
    let mut cpuid = CpuId::new();

    match cpuid.get_vendor_info() {
        Some(vi) =>
            if vi.as_string() == "AuthenticAMD" {
                BRAND == "AMD";
                x86_printk!("Cpu: {}", vi.as_string());
            } else if vi.as_string() == "GenuineIntel" {
                x86_printk!("cpu: {}", vi.as_string());
                BRAND == "Intel";
            },
        None => set_info("not ok")
    }
}
