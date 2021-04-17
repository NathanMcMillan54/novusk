pub static mut ARCHITECTURE: &str = "";

unsafe fn x86() -> bool {
    ARCHITECTURE = "x86";
    return true;
}

unsafe fn x86_64() -> bool {
    ARCHITECTURE = "x86_64";
    return true
}

fn invalid() -> bool {
    return false
}

pub unsafe fn validate_cpu() -> bool {
    #[cfg(target_arch = "x86_64")]
    return x86_64();

    #[cfg(target_arch = "x86")]
    return x86();

    #[cfg(not(target_arch = "x86_64"))]
    return invalid();
}
