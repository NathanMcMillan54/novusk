use ethernet::EthNetDriver;
pub use stm_eth::StmEth;

pub(crate) fn is_supported() -> bool {
    #[cfg(any(feature = "stm32f407"))]
    return true;

    #[cfg(any(feature = "stm32f401"))]
    return false;
}

pub fn net_init() {
    let mut eth = StmEth::new();

    if is_supported() {
        eth.init();
    } else { printk!("Sorry! Driver: {} by: {} is not supported on this device", eth.driver.name, eth.driver.author); }
}
