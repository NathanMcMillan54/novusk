use novuskinc::platform::{early_device_init, device_init, DEVICE_INIT_ERRORS};
use crate::kernel::irq::aarch64_irq_init;
use crate::kernel::utils::el;

pub unsafe fn aarch64_kernel_init() {
    // aarch64_irq_init();

   // let dev_init = device_init();

    if dev_init == 0 {

    } else {

    }

    /*syscalls_init();
    kinfo!("System calls initialized\n");*/


    extern "C" {
        fn kernel_main();
    }
}
