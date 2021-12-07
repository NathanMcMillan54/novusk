use multiboot::multiboot_init;
use crate::x86_printk;
use crate::boot::main::main;
use crate::mm::early_memory_init;

#[no_mangle]
pub unsafe extern "C" fn grub_start_novusk(bootinfo_address: usize) -> ! {
    x86_printk!("Booted with GRUB\n\n");

    multiboot_init(bootinfo_address);
    kinfo!("GRUB bootloader initialized\n");

    main();
}
