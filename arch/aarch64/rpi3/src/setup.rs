use arm_lib::include::asm::wfe;

extern "C" {
    fn aarch64_kernel_init();
}

pub unsafe fn rpi_setup() -> ! {
    aarch64_kernel_init();
    wfe()
}
