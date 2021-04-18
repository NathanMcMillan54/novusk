pub static mut KTIME: f64 = 0.0;

pub async unsafe fn time_init() {
    KTIME = KTIME + 0.001;
}

#[no_mangle]
pub unsafe extern "C" fn x86_time() -> f64 {
    return KTIME;
}
