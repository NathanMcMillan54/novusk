use novuskinc::drivers::{add_driver, Driver};

pub const UART0: *mut u8 = 0x4000_C000 as *mut u8;
pub const UART1: *mut u8 = 0x4000_D000 as *mut u8;
pub const UART2: *mut u8 = 0x4000_E000 as *mut u8;

gen_simpleuart!();

pub(crate) unsafe fn lm3s6965_simpleuart_init() {
    KERNEL_SIMPLEUART.name = "LM3s6965 SimpleUart";
    KERNEL_SIMPLEUART.output_addr = UART0;
    KERNEL_SIMPLEUART.input_addr = UART0;

    add_driver(&KERNEL_SIMPLEUART as &'static dyn Driver);
}
