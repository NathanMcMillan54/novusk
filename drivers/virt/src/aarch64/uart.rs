use novuskinc::kernel::types::KernelFunctionName;

gen_simpleuart!();

unsafe fn virt_simpleuart_init() -> u8 {
    KERNEL_SIMPLEUART.name = "Virt SimpleUart";
    KERNEL_SIMPLEUART.output_addr = 0x0900_0000 as *mut u8;
    KERNEL_SIMPLEUART.input_addr = 0x0900_0000 as *mut u8;

    0
}

define_kernel_function!(KernelFunctionName::early_serial_init, -> u8, virt_simpleuart_init);
