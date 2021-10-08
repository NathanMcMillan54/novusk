use crate::x86_printk;
use crate::boot::main::main;
use crate::mm::boot::set_boot_mem_map;

#[no_mangle]
pub unsafe extern "C" fn grub_start_novusk() -> ! {
    x86_printk!("Booted with GRUB");

    //set_boot_mem_map();

    main();
    loop {  }
}
