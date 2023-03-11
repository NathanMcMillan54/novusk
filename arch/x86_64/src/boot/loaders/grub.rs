use multiboot::multiboot_init;
use crate::boot::main::main;
use crate::mm::test_allocator;

#[no_mangle]
pub unsafe extern "C" fn grub_start_novusk(bootinfo_address: usize) -> ! {
    // early_printk!("Booted with GRUB\n\n");

    multiboot_init(bootinfo_address);
    /*kinfo!(KStatus {
        status: "ok",
        should_panic: false,
        panic_message: None,
        main_message: "Grub bootloader initialized",
        messages: None,
    });

    early_printk!("Testing allocator...\n");*/
    test_allocator();

    main();
}
