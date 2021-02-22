pub static SLEEP_TIME: i32 = 7500000;

extern "C" { pub fn time_init(); }
extern "C" { pub fn kernel_time() -> i32; }
