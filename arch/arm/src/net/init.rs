#[cfg(any(feature = "stm32f401", feature = "stm32f407"))]
use stmd::board::*;

#[cfg(not(any(feature = "stm32f401", feature = "stm32f407")))]
fn is_supported() -> bool {
    return false;
}

#[cfg(not(any(feature = "stm32f401", feature = "stm32f407")))]
fn net_init() {

}

pub(crate) fn arm_ethernet_init() {
    if is_supported() {
        net_init();
    } else { printk!(""); }
}

#[link_name = "wireless_init"]
pub(crate) fn arm_wrireless_init() {

}

define_ethernet_init!(arm_ethernet_init);
