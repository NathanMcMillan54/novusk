use bcm_soc::uart::{bcm2837_uart_init, Bcm2837Uart};
use core::sync::atomic::{compiler_fence, Ordering};
use novuskinc::drivers::Driver;
use novuskinc::kernel::types::KernelFunctionName;
use console::{KERNEL_CONSOLE, MainKernelConsole};
use mailbox::MailBox;
use crate::DEVICE_DRIVERS;

pub mod led;
pub mod serial;

unsafe fn early_rpi3_init() -> u8 {
    DEVICE_DRIVERS.add_driver(&serial::KERNEL_SIMPLEUART as &'static dyn Driver);
    bcm2837_uart_init();

    KERNEL_CONSOLE.printing_method = Some(&Bcm2837Uart);
    DEVICE_DRIVERS.add_driver(&KERNEL_CONSOLE as &'static dyn Driver);

    0
}

define_kernel_function!(KernelFunctionName::early_device_init, -> u8, early_rpi3_init);

unsafe fn rpi3_init() -> u8 {

    0
}

define_kernel_function!(KernelFunctionName::device_init, -> u8, rpi3_init);
