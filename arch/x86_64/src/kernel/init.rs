use super::{tests, time};

pub unsafe fn init() {
    time::time_init();
    tests::tests_init();
}
