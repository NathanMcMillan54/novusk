mod fmt;
use fmt::fmt_test;

use super::error::{test_error};

pub unsafe fn tests_init() {
    // There is literally no way the format test can fail <--- That's a joke it will eventually and that's a problem
    if fmt_test() == 0 {
        test_error("Format text test", true);
    }
}
