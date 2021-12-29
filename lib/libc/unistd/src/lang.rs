use core::alloc::{GlobalAlloc, Layout};
use core::panic::PanicInfo;

#[panic_handler]
fn _panic(_info: &PanicInfo) -> ! {
    loop {  }
}

#[alloc_error_handler]
fn oom(_layout: Layout) -> ! {
    loop {  }
}

struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        0 as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {

    }
}

#[global_allocator]
static ALLOC: Allocator = Allocator;
