use core::alloc::Layout;

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("Alloc error, Layout: {:?}", layout);
}
