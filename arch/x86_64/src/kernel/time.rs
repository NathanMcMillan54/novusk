pub static mut KTIME: f64 = 0.0;

pub unsafe fn time_init() {
    KTIME = KTIME + 0.001;
}

pub extern "C" fn current_time() -> f64 {
    unsafe { KTIME }
}
