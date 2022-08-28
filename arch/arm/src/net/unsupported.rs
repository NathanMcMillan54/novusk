use novuskinc::kernel::types::KernelFunctionName;

fn arm_net_init() -> u8 {
    0
}

define_kernel_function!(KernelFunctionName::net_init, -> u8, arm_net_init);