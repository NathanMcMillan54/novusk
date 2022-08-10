use novuskinc::kernel::types::KernelFunctionName;

fn aarch64_net_init() -> u8 {
    0
}

define_kernel_function!(KernelFunctionName::net_init, -> u8, aarch64_net_init);
