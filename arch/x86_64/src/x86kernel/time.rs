pub static mut KTIME: f64 = 0.0;

pub async unsafe fn time_init() {
    KTIME = KTIME + 1.0;
    loop { current_time(); }
}

pub unsafe fn current_time() -> f64 {
    KTIME = KTIME + 0.01;
    return KTIME;
}
