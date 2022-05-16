use crate::early_printk;
use novuskinc::platform::set_platform_drivers;
use printk::printk_init;

#[no_mangle]
pub unsafe extern "C" fn arch_prepare_init() {
    set_platform_drivers();

    printk_init("Console Driver");
}
