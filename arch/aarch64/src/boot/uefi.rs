use crate::boot::setup::setup;
use crate::drivers::uefi_init;
use uefi::Handle;
use uefi::table::{Boot, SystemTable};

#[no_mangle]
pub unsafe extern "C" fn efi_main(image: Handle, system_table: SystemTable<Boot>) -> ! {
    uefi_init(image, system_table);
    setup()
}
