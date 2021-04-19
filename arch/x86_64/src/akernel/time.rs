// This is a fake clock like in v1 - v1.1
// Because the kernel takes a while to load start at 1
pub static mut KTIME: f64 = 1.0;

pub async unsafe fn time_init() {
    KTIME = KTIME + 0.01;
}

pub unsafe fn current_time() -> f64 {
    x86_time()
}

#[no_mangle]
pub unsafe extern "C" fn x86_time() -> f64 {
    KTIME = KTIME + 0.01;
    return KTIME;
}
