use core::alloc::Layout;

#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
    unsafe { core::ptr::write_volatile(0x4000_C000 as *mut u8, b'o'); }
    panic!("Alloc OOM, layout: {:?}", layout);
}
