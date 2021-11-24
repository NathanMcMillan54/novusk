use crate::aarch64_printk;
use init::kmain::kernel_init;
use super::device;
use libbmu::bmu_init;

pub unsafe fn aarch64_init() {
    setup_device();

    bmu_init();
}

fn setup_device() {
    let (success, device) = device::initialize_device();

    if success.is_err() {
        panic!("Error while initializing device: {:?}", success.err());
    }

    aarch64_printk!("Initialized {}", device);
}
