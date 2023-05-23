use embedded_alloc::Heap;

#[global_allocator]
pub static ALLOCATOR: Heap = Heap::empty();

pub unsafe fn allocator_init(mem_start: usize, mem_size: usize) {
    ALLOCATOR.init(mem_start, mem_size);
}
