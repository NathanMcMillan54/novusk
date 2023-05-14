use core::alloc::{GlobalAlloc, Layout};
use core::mem::MaybeUninit;

cfg_if! {
    if #[cfg(feature = "alloc-cortex-m")] {
        use alloc_cortex_m::CortexMHeap;

        #[global_allocator]
        pub static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

        pub unsafe fn allocator_init() {
            const CM_HEAP_SIZE: usize = 1024;
            static mut HEAP: [MaybeUninit<u8>; CM_HEAP_SIZE] = [MaybeUninit::uninit(); CM_HEAP_SIZE];
            ALLOCATOR.init(HEAP.as_ptr() as usize, CM_HEAP_SIZE);
        }
    } else if #[cfg(feature = "nmallocator")] {
        pub use nmallocator::ALLOCATOR;

        pub fn allocator_init() {

        }
    } else {
        struct ArmAllocator;

        unsafe impl GlobalAlloc for ArmAllocator {
            unsafe fn alloc(&self, layout: Layout) -> *mut u8 { todo!() }
            unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) { todo!() }
        }

        #[global_allocator]
        static mut ALLOCATOR: ArmAllocator = ArmAllocator;
    }
}

#[cfg(feature = "allocator")]
pub fn test_allocator() {
    let mut test_vec = vec![];

    for i in 0..1024 {
        test_vec.push(1);
    }

    test_vec.push(2);

    assert_eq!(1025, test_vec.len());
}
