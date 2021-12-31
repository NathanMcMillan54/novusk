use ethernet::EthNetDriver;
pub use stm_eth::StmEth;

pub fn is_supported() -> bool {
    #[cfg(any(feature = "stm32f407"))]
    return true;

    #[cfg(all(not(feature = "stmf407")))]
    return false;

    return true;
}

pub fn net_init() {
    let mut eth = StmEth::new();
    eth.init();
}
