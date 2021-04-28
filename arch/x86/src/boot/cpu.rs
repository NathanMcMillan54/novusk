use super::boot::{die};
use raw_cpuid::CpuId;

pub static mut ARCH: &'static str = "";

unsafe fn check_intel() {

}

unsafe fn check_amd() {

}

pub unsafe fn validate_cpu() -> bool {
    let cpuid = CpuId::new();
    match cpuid.get_vendor_info() {
        Some(vi) => {
            if vi.as_string() == "GenuineIntel" {
                check_intel();
            } else if vi.as_string() == "AuthenticAmd" {
                check_amd();
            }
        }
        None => {
            return false;
        }
    }
    return true;
}
