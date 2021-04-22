pub const VERSION_ONE: i32 = 1;
pub const VERSION_TENTHS: i32 = 1;
pub const VERSION_HUNDREDTHS: i32 = 0;
pub const VERSION_NAME: &str = "New Kernel";

pub unsafe fn version_init() -> i32 {
    printk!("|                  |\n| Version: {}.{}.{}   |\n| {}       |\n|__________________|", VERSION_ONE, VERSION_TENTHS, VERSION_HUNDREDTHS, VERSION_NAME);
    return VERSION_ONE + VERSION_TENTHS + VERSION_HUNDREDTHS;
}

#[no_mangle]
pub extern "C" fn version_num() -> i32 {
    return "1.1.0".parse().unwrap();
}

#[no_mangle]
pub extern "C" fn version_name() -> &'static str {
    return VERSION_NAME;
}
