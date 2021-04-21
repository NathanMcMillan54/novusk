use super::{modules, syscalls, tests, time};

pub unsafe fn init() {
    time::time_init();
    printk!("x86_64 kernel init");
    kinfo!("Kernel time initialized");
    tests::tests_init();
    kinfo!("Kernel tests finished");
    let test_info = tests::test_info();
    printk!("   {}/1 tests failed", test_info);
    modules::x86_64_kernel_modules_init();
    kinfo!("x86_64 kernel modules finished");
}
