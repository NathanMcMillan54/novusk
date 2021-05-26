use core::ptr::NonNull;
use uefi_services::system_table;
use uefi::table::{Boot, SystemTable};

#[no_mangle]
pub extern "C" fn st() -> NonNull<SystemTable<Boot>> {
    return uefi_services::system_table();
}
