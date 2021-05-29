use crate::st;
use uefi::Status;
use uefi::table::runtime::ResetType;

pub struct UefiPower;

impl UefiPower {
    pub unsafe fn shutdown(&mut self) -> ! {
        let rs = st().as_ref().runtime_services();
        rs.reset(ResetType::Shutdown, Status::SUCCESS, None)
    }

    // This doesn't restart at the time it was called, that needs to be fixed soon
    pub unsafe fn reboot(&mut self) -> ! {
        let rs = st().as_ref().runtime_services();
        rs.reset(ResetType::PlatformSpecific, Status::SUCCESS, None)
    }
}
