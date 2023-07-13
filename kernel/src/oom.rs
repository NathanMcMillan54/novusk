use core::alloc::Layout;

fn _oom(layout: &Layout) -> ! {
    panic!("Out of Memory, allocator failed");
}
