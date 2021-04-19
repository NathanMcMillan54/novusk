use super::{tests, time};

pub unsafe fn init() {
    time::time_init();
    tests::tests_init();
    info!("Kernel time started\n");
    info!("Early kernel tests finished\n");
}
