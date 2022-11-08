#[no_mangle]
pub unsafe extern "C" fn add_timer(timer_name: &'static str) {
    for i in 0..KERNEL_TIMERS.timers.len() {
        if KERNEL_TIMERS.timers[i].0 == "" {
            KERNEL_TIMERS.timers[i] = (timer_name, 0.0);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn update_timer(timer_name: &'static str, update: f64) {
    for i in 0..KERNEL_TIMERS.timers.len() {
        if KERNEL_TIMERS.timers[i].0 == timer_name {
            KERNEL_TIMERS.timers[i].1 += update;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_timer_value(timer_name: &'static str) -> f64 {
    for i in 0..KERNEL_TIMERS.timers.len() {
        if KERNEL_TIMERS.timers[i].0 == timer_name {
            return KERNEL_TIMERS.timers[i].1;
        }
    }

    0.0
}

pub struct KernelTimers {
    pub timers: &'static mut [(&'static str, f64); 10],
}

pub static mut KERNEL_TIMERS: KernelTimers = KernelTimers {
    timers: &mut [("", 0.0); 10],
};
