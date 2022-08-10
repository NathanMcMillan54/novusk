use crate::StmEth;
use ethernet::{EthNetDriver, EthNet};

fn is_supported() -> bool {
    #[cfg(feature = "unsupported_stm32fxxx")]
    return false;

    #[cfg(feature = "stm32f407")]
    return true;
}

fn stm_ethernet_init() -> u8 {
    let mut eth = StmEth::new();
    let (name, author) = eth.driver.driver_info();

    if is_supported() {
        eth.init();
        return 0;
    } else {
        printk!("Driver: {} by: {} is not supported\n", name, author);
        return 1;
    }
}

define_kernel_function!(KernelFunctionName::net_init, -> u8, stm_ethernet_init);
