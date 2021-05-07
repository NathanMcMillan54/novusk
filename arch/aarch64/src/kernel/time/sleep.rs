pub unsafe fn sleep(sec: i32) {
    let sleep_time= sec * 100000;
    let mut i = 0;
    while i != sleep_time {
        asm!("");
        i = i + 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn sleepm(mil: i32) {
    // I don't know why but when you make sleep_time = mil * 1000 it takes forever to finish
    // It seems more like a second when it's set to 175
    let sleep_time = mil * 175;
    let mut i = 0;
    while i != sleep_time {
        asm!("");
        i = i + 1;
    }
}
