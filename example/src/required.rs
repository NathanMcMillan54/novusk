#[no_mangle]
pub unsafe extern "C" fn main_test() -> i32 { return 0; }
#[no_mangle]
pub extern "C" fn is_os() -> bool { false }
#[no_mangle]
pub extern "C" fn os_name() -> &'static str { "none" }
#[no_mangle]
pub extern "C" fn is_initramfs() -> bool { false }
#[no_mangle]
pub extern "C" fn device_name() -> &'static str { "default" }
#[no_mangle]
pub unsafe extern "C" fn initramfs_main() { return; }
#[no_mangle]
pub extern "C" fn kernfs_name() -> &'static str { "kernel filesystem" }
#[no_mangle]
pub unsafe extern "C" fn kernfs_init() { return; }
