#[no_mangle]
pub extern "C" fn android_start() -> ! {
    loop {  }
}

#[no_mangle]
pub extern "C" fn android_version() -> i32 {
    return 1;
}
