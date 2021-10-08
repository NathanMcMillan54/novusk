use bootloader::BootInfo;
use crate::boot::main::main;

#[no_mangle]
pub unsafe extern "C" fn bootloader_start_novusk(bootinfo: &'static BootInfo) -> ! {
    main(bootinfo);
}
