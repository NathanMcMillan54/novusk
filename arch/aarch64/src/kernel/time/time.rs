pub static SLEEP_TIME: i32 = 7500000;

extern "C" { pub fn time_init() -> f32; }
extern "C" { pub fn kernel_time() -> f32; }
