use super::KTIME;

static mut UPDATE: f32 = 1.1;

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

pub fn time_reinit() -> f32 {
    unsafe {
        KTIME = 1.0;
        UPDATE = 1.0;
        return KTIME;
    }
}
