#![no_std]

#[macro_use] extern crate novuskinc;

pub mod consts;
pub mod init;
pub mod io;

use consts::*;

pub struct RpiSdCard;

impl RpiSdCard {
    pub fn new() -> Self {
        return RpiSdCard;
    }
}
