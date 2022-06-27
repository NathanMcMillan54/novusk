use core::fmt::{Arguments, Result, Write};
use libc::printf;

pub struct TestLogger;

impl TestLogger {
    pub fn new() -> Self {
        return TestLogger;
    }
}

impl Write for TestLogger {
    fn write_str(&mut self, s: &str) -> Result {
        unsafe { printf(s.as_ptr() as *const _); }
        Ok(())
    }
}

pub fn _log(msg: Arguments) {
    let mut logger = TestLogger::new();

    logger.write_fmt(format_args!("{}{}", msg, "\0"));
}

#[macro_export]
macro_rules! test_log {
    ($($args:tt)*) => {
        $crate::log::_log(format_args!($($args)*));
    };
}
