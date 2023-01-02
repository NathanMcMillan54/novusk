use novuskinc::kernel::types::KernelFunctionName;

pub mod uart;

unsafe fn early_virt_init() -> u8 {

    0
}

unsafe fn virt_init() -> u8 {
    0
}

define_kernel_function!(KernelFunctionName::early_device_init, -> u8, early_virt_init);
define_kernel_function!(KernelFunctionName::device_init, -> u8, virt_init);
