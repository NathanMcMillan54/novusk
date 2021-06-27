#[cfg(target_arch = "x86_64")]
mod x64;

#[cfg(target_arch = "x86")]
mod x86;


pub static mut BRAND: &'static str = "Unknown";

pub unsafe fn get_cpuid() {
    #[cfg(target_arch = "x86_64")]
    x64::get_cpuid();

    #[cfg(target_arch = "x86")]
    x86::get_cpuid();
}
