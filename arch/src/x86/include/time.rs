pub fn sleep(seconds: i32) {
    let mut x = 0;
    let time = seconds * 100000000;
    loop {
        x = x + 1;
        if x == time {
            break;
        }
    }
}

pub fn kernel_time() -> i32 {
    let mut x = 0;
    x = x + 1;
    if x > 100000000 {
        x = x / 10;
    }
    return x;
}
