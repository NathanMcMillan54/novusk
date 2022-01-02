pub mod allocator;

use allocator::allocator_test;

pub fn memory_init() -> Result<(), i32> {
    if allocator_test() {
        return Ok(());
    } else { return Err(1); }
}
