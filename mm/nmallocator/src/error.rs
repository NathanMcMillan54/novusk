use core::alloc::Layout;

#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
    panic!("Alloc OOM, layout: {:?}", layout);
}
