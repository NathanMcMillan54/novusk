use novuskinc::kernel::{arch_prepare_init, kernel_init, setup_arch};

pub unsafe fn start_aarch64_kernel() {
    setup_arch();
    arch_prepare_init();
    kernel_init();

    panic!("Aarch64 kernel ended")
}
