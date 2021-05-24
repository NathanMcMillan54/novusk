use core::ptr::NonNull;
use uefi::prelude::{SystemTable, Boot};

#[no_mangle]
pub unsafe extern "C" fn st() -> NonNull<SystemTable<Boot>> {
    return uefi_services::system_table();
}
