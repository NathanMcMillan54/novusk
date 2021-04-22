mod os;

static mut INFO: i32 = 0;
pub const TESTS: i32 = 1;

unsafe fn error(msg: &str) {
    printk!("{}", msg);
    INFO = INFO + 1;
}

pub unsafe fn tests_init() {
    if os::main_test() == 1 {
        error("OS test failed\n    This might cause a lot of problems later\n");
    }
}

pub unsafe fn test_info() -> i32 {
    return INFO;
}
