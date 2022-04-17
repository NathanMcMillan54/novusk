use novuskinc::timer::device_timer_init;

static mut CURRENT_TIME: u64 = 0;

pub unsafe fn timer_init() {
    device_timer_init();
}
