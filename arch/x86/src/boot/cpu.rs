pub unsafe fn check_arch() -> bool {
    #[cfg(target_arch = "x86")]
    return true;

    #[cfg(target_arch = "x86_64")]
    return true;

    #[cfg(not(target_arch = "x86_64"))]
    return false;

    #[cfg(target_arch = "x86")]
    return false;
}
