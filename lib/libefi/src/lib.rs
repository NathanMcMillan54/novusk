#![no_std]

use core::ptr::NonNull;
use uefi::Status;
use uefi::table::{Boot, SystemTable};
use uefi::table::runtime::ResetType;

extern "C" {
    pub fn st() -> NonNull<SystemTable<Boot>>;
}

#[no_mangle]
pub unsafe extern "C" fn shutdown() -> ! {
    let rs = st().as_ref().runtime_services();
    rs.reset(ResetType::Shutdown, Status::SUCCESS, None);
}

// This doesn't restart at the time it was called, that needs to be fixed soon
#[no_mangle]
pub unsafe extern "C" fn reboot() -> ! {
    let rs = st().as_ref().runtime_services();
    rs.reset(ResetType::PlatformSpecific, Status::SUCCESS, None);
}
