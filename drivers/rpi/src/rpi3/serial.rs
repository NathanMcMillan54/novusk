use novuskinc::kernel::types::KernelFunctionName;

gen_simpleuart!();

unsafe fn simple_uart_setup() -> u8 {
    KERNEL_SIMPLEUART.name = "RPi3 SimpleUart";
    KERNEL_SIMPLEUART.input_addr = 0x3F00_0000 as *mut u8;
    KERNEL_SIMPLEUART.output_addr = 0x3F20_1000 as *mut u8;

    0
}

define_kernel_function!(KernelFunctionName::early_serial_init, -> u8, simple_uart_setup);
