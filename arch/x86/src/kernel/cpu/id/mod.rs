#[cfg(target_arch = "x86_64")]
mod x64;

pub static mut BRAND: &'static str = "Unknown";

pub unsafe fn get_cpuid() {
    #[cfg(target_arch = "x86_64")]
    x64::get_cpuid();
}
