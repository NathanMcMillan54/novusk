pub static mut CPU_TIME: u64 = 0;

pub fn cpu_time() -> u64 {
    unsafe { return CPU_TIME; }
}

pub fn update_cpu_time(update: u64) {
    unsafe { CPU_TIME = update; }
}
