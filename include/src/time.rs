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
