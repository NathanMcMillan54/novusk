use core::ptr::NonNull;
use uefi::table::{Boot, SystemTable};
use uefi_services::system_table;

#[no_mangle]
pub extern "C" fn st() -> NonNull<SystemTable<Boot>> {
    return system_table();
}
