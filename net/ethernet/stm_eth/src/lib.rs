#![no_std]

#[macro_use] extern crate ethernet;
#[macro_use] extern crate novuskinc;

use ethernet::*;

pub mod driver;
pub mod init;

#[derive(Copy, Clone)]
pub struct StmEth {
    pub driver: EthNet,
    connection: bool,
}

impl StmEth {
    pub fn new() -> Self {
        let mut eth = EthNet::new("STM32 ethernet", "Nathan McMillan");
        return StmEth {
            driver: eth,
            connection: eth.is_connection()
        };
    }
}
