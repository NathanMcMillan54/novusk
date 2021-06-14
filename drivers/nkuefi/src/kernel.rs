extern "C" {
    fn aarch64_kernel_init() -> !;
    fn x86_kernel_init() -> !;
}

pub unsafe fn start_novusk() -> ! {
    #[cfg(target_arch = "x86_64")]
    x86_kernel_init();

    #[cfg(target_arch = "aarch64")]
    aarch64_kernel_init();
}
