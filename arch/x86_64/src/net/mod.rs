use novuskinc::kernel::types::KernelFunctionName;

fn x86_64_net_init() -> u8 {
    0
}

define_kernel_function!(KernelFunctionName::net_init, -> u8, x86_64_net_init);
