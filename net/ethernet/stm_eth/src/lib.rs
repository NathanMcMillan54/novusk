#![no_std]

use ethernet::*;

pub mod driver;

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
