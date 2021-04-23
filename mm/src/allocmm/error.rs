use core::alloc::Layout;

#[alloc_error_handler]
unsafe fn alloc_error(_layout: Layout) -> ! {
    printk!("Alloc memory error");
    loop {  }
}
