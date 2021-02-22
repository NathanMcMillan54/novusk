pub static SLEEP_TIME: i32 = 1000000;

extern "C" { pub(crate) fn kernel_time() -> f32; }
extern "C" { pub(crate) fn time_init(); }
