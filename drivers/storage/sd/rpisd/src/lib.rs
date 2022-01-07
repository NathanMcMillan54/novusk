#![no_std]

#[macro_use] extern crate novuskinc;

pub mod init;
pub mod io;

pub struct RpiSdCard;

impl RpiSdCard {
    pub fn new() -> Self {
        return RpiSdCard;
    }
}
