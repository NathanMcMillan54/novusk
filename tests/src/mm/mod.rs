use nmallocator::tests::no_alloc_error;
use crate::test_log;

pub fn run_mm_tests() {
    test_log!("Running 'no_alloc_error' from 'nmallocator'\n");
    no_alloc_error();
    test_log!("'no_alloc_error' was successful\n");
}
