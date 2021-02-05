use core::alloc::Layout;

#[alloc_error_handler]
fn alloc_error_handler(_layout: Layout) -> ! {
    loop {  }
}
