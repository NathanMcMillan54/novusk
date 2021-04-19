use super::{tests, time};

pub unsafe fn init() {
    time::time_init();
    tests::tests_init();
    info!("Kernel time initialized\n");
    info!("Early kernel tests finished\n");
    x86_kernel_modules_init();
    info!("x86_64 kernel modules initialized\n");
}

extern "C" { fn x86_kernel_modules_init(); }
