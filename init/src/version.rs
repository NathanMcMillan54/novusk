pub const VERSION_ONE: i32 = 1;
pub const VERSION_TENS: i32 = 1;
pub const VERSION_HUNDREDS: i32 = 0;
pub const VERSION_NAME: &str = "New Kernel";

pub unsafe fn version_init() -> i32 {
    printk!("|                  |\n");
    printk!("| Version: {}.{}.{}   |\n", VERSION_ONE, VERSION_TENS, VERSION_HUNDREDS);
    printk!("| {}       |\n", VERSION_NAME);
    printk!("|__________________|\n");
    return VERSION_ONE + VERSION_TENS + VERSION_HUNDREDS;
}

#[no_mangle]
pub extern "C" fn version_num() -> i32 {
    return "1.1.0".parse().unwrap();
}

#[no_mangle]
pub extern "C" fn version_name() -> &'static str {
    return VERSION_NAME;
}
