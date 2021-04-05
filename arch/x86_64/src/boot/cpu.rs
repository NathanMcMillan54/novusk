pub unsafe fn validate_cpu() -> bool {
    #[cfg(target_arch = "x86_64")]
    return true;

    #[cfg(not(target_arch = "x86_64"))]
    return false;
}
