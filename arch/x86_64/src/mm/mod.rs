use bootloader::BootInfo;
use crate::x86_printk;
use x86_64::VirtAddr;

// A lot of this is from Philipp Oppermann's blog os
#[cfg(feature = "bootloader_rs")]
pub mod heap_alloc;

#[cfg(feature = "grub")]
pub mod wee_alloc;

use kinfo::status::set_status;

#[cfg(feature = "bootloader_rs")]
pub unsafe fn early_memory_init(bootinfo: &'static BootInfo) {
    use heap_alloc::{alloc_heap_init, memory::{BootInfoFrameAllocator, memory_init}};

    let mut mem_map = memory_init(VirtAddr::new(bootinfo.physical_memory_offset));
    let mut frame_allocator = BootInfoFrameAllocator::init(&bootinfo.memory_map);

    let alloc_init = match alloc_heap_init(&mut mem_map, &mut frame_allocator) {
        Ok(..) => {
            x86_printk!("Alloc heap memory initialized successfully\n");
        }

        Err(..) => {
            set_status("not ok");
            x86_printk!("Alloc heap memory initialized unsuccessfully, this will likely cause errors in the future\n");
        }
    };
}

#[cfg(feature = "grub")]
pub fn early_memory_init() {
    let mut a_vec = vec![0];

    for i in 0..1000 {
        a_vec.push(i);
    }
}
