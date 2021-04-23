use core::alloc::Layout;

#[alloc_error_handler]
unsafe fn alloc_error(_layout: Layout) -> ! {
    printk!("|---------------------|\n| Alloc memory error |\n|-----------------------|");
    printk!("   Ran out of memory or kernel tried to use unusable memory");
    kinfo!("Ending kernel for now");
    loop {  }
}
