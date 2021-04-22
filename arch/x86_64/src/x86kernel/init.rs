use super::{modules, syscalls, tests, time, kernel_init};

pub unsafe fn init() {
    time::time_init();
    printk!("Starting kernel...\n");
    kinfo!("Kernel time initialized");
    tests::tests_init();
    kinfo!("Kernel tests finished");
    let test_info = tests::test_info();
    let test = tests::TESTS;
    printk!("   {}/{} tests failed", test_info, test);
    modules::x86_64_kernel_modules_init();
    kinfo!("x86_64 kernel modules finished");
    kernel_init();
}
