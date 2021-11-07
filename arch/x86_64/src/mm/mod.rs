use bootloader::BootInfo;
use crate::x86_printk;
use x86_64::VirtAddr;

// A lot of this is from Philipp Oppermann's blog os
pub mod allocator;

pub mod boot;
pub mod memory;

use allocator::alloc_heap_init;
use boot::{BOOT_PHYS_MEM_OFFSET};
use memory::{memory_init, BootInfoFrameAllocator};
use kinfo::status::set_status;

pub unsafe fn early_memory_init(bootinfo: &'static BootInfo) {
    let mut mem_map = memory_init(VirtAddr::new(BOOT_PHYS_MEM_OFFSET as u64), 1649267441664 as u64);
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
