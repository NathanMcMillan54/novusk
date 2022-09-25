use init::kmain;
use kinfo::{InfoDisplay, status::KStatus};
use novuskinc::platform::{early_device_init, device_init, DEVICE_INIT_ERRORS};
use setup::after_kernel_setup;
use crate::include::sys::setup::syscalls_init;

pub unsafe fn aarch64_kernel_init() {
    let dev_init = device_init();

    if dev_init == 0 {
        kinfo!(KStatus {
            status: "ok",
            should_panic: false,
            panic_message: None,
            main_message: "Device initialized",
            messages: None,
        });
    } else {
        kinfo!(KStatus {
            status: "not ok",
            should_panic: true,
            panic_message: Some(DEVICE_INIT_ERRORS[dev_init as usize]),
            main_message: "Device initialization failed",
            messages: None,
        });
    }

    /*syscalls_init();
    kinfo!("System calls initialized\n");*/

    kmain::kernel_init();
    kinfo!(KStatus {
        status: "ok",
        should_panic: false,
        panic_message: None,
        main_message: "Novusk initialized",
        messages: None,
    });

    extern "C" {
        fn kernel_main();
    }

    early_printk!("Starting main...\n");
    after_kernel_setup();
    kinfo!(KStatus {
        status: "ok",
        should_panic: false,
        panic_message: None,
        main_message: "After kernel initialized",
        messages: None,
    });

    kernel_main();
}
