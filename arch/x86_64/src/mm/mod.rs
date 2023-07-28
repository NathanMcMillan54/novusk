use embedded_alloc::Heap;

pub mod memory;

#[global_allocator]
static mut ALLOCATOR: Heap = Heap::empty();
