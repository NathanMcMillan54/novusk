pub static mut KERNEL_TIME: u64 = 0;

pub fn kernel_time() -> u64 {
    unsafe { return KERNEL_TIME; }
}

pub fn update_kernel_time(update: u64) {
    unsafe { KERNEL_TIME = KERNEL_TIME + update; }
}
