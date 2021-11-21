use crate::aarch64_printk;
use init::kmain::kernel_init;
use libbmu::bmu_init;

pub unsafe fn aarch64_init() {
    bmu_init();
}
