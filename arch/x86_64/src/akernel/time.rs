pub static mut KTIME: f64 = 0.0;

pub unsafe fn time_init() -> f64 {
    KTIME = KTIME + 1.0;
    current_time();
    return KTIME;
}

pub unsafe fn current_time() -> f64 {
    KTIME = KTIME + 0.01;
    return KTIME;
}
