#![no_std]

#[macro_use] extern crate novuskinc;

pub mod consts;
pub mod init;
pub mod io;

#[path = "../../sd_trait.rs"]
pub mod sd_trait;

use consts::*;
use sd_trait::SdCard;

pub struct RpiSdCard;

impl RpiSdCard {
    pub fn new() -> Self {
        return RpiSdCard;
    }
}

impl SdCard for RpiSdCard {
    const SD_OK: u32 = 1;
    const SD_ERR: u32 = 2;

    fn status(&self, mask: i32) -> u32 {
        // For now return SD_OK
        return RpiSdCard::SD_OK;
    }
}
