use super::KTIME;
static mut UPDATE: f32 = 0.2;

#[no_mangle]
pub extern "C" fn time_init() -> f32 {
    unsafe {
        KTIME = 0.1;
        return KTIME;
    }
}

#[no_mangle]
pub extern "C" fn kernel_time() -> f32 {
    unsafe {
        KTIME += UPDATE;
        return KTIME;
    }
}

#[no_mangle]
pub extern "C" fn add_0_1() {
    unsafe { KTIME += 0.1; }
}
