use ethernet::EthNetDriver;
pub use stm_eth::StmEth;

pub fn is_supported() -> bool {
    #[cfg(any(feature = "stm32f407"))]
    return true;

    #[cfg(any(feature = "stm32f401"))]
    return false;
}

pub fn net_init() {
    let mut eth = StmEth::new();
    eth.init();
}
