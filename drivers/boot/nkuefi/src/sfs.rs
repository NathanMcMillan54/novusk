use uefi::prelude::BootServices;
use uefi::proto::media::fs::SimpleFileSystem;

pub struct SfsInterface;

impl SfsInterface {
    pub fn new() -> Self {
        return SfsInterface;
    }

    pub fn get_sfs(&self, bt: &BootServices) -> &mut SimpleFileSystem {
        let sfs = unsafe { &mut *bt.locate_protocol::<SimpleFileSystem>().unwrap().unwrap().get() };

        return sfs;
    }
}
