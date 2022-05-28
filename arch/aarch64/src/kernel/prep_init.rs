use crate::early_printk;
use printk::printk_init;

#[no_mangle]
pub unsafe extern "C" fn arch_prepare_init() {
    printk_init("Console Driver");
}
