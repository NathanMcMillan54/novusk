#[cfg(target_arch = "x86_64")]
mod x64;

#[cfg(target_arch = "x86")]
global_asm!(include_str!("x86_id.S"));

extern "C" {
    fn get_x86_cpuid();
}

pub static mut BRAND: &'static str = "Unknown";

pub unsafe fn get_cpuid() {
    #[cfg(target_arch = "x86_64")]
    x64::get_cpuid();

    #[cfg(target_arch = "x86")]
    get_x86_cpuid();
}
