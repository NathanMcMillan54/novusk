use bootloader::BootInfo;
use crate::x86_printk;
use crate::boot::main::main;
use crate::mm::{early_memory_init, heap_alloc::allocator_init};

#[no_mangle]
pub unsafe extern "C" fn bootloader_start_novusk(bootinfo: &'static BootInfo) -> ! {
    x86_printk!("Booted with bootloader rs\n");

    early_memory_init(bootinfo);
    allocator_init();
    kinfo!("Early memory initialized\n");
    x86_printk!("    Allocator initialized\n");
    x86_printk!("    Testing alloc...\n\n");
    vec![1, 1].push(1);

    main();
}
