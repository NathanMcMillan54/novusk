use bootloader::BootInfo;
use kinfo::status::KStatus;
use crate::boot::main::main;
use crate::mm::test_allocator;
use crate::kinfo::InfoDisplay;

#[no_mangle]
pub unsafe extern "C" fn bootloader_start_novusk(bootinfo: &'static BootInfo) -> ! {
    early_printk!("Booted with bootloader rs\n");

    early_printk!("Testing allocator...\n");
    test_allocator();

    main();
}
