use super::{currentTime, startTime};

pub(crate) fn time_init() {
    unsafe { startTime() }
}

pub static SLEEP_TIME: i32 = 7500000;
