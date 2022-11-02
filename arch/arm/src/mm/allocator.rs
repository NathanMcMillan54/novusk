use core::alloc::Layout;
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

        #[alloc_error_handler]
        fn alloc_error(layout: Layout) -> ! {
            panic!("OOM");
        }
    } else if #[cfg(feature = "nmallocator")] {
        pub use nmallocator::ALLOCATOR;
    }
}
