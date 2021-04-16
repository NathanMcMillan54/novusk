pub static mut ARCHITECTURE: &str = "";

pub unsafe fn validate_cpu() -> bool {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    return true;

    #[cfg(not(target_arch = "x86_64", target_arch = "x86"))]
    return false;
}
