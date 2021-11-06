use multiboot::multiboot_init;
use crate::x86_printk;
use crate::boot::main::main;

#[no_mangle]
pub unsafe extern "C" fn grub_start_novusk(bootinfo_address: usize) -> ! {
    x86_printk!("Booted with GRUB\n");

    multiboot_init(bootinfo_address);
    kinfo!("GRUB bootloader initialized");

    loop {

    }

    main();
}
