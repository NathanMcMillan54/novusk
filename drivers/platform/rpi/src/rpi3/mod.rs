use novuskinc::kernel::types::KernelFunctionName;

unsafe fn rpi3_init() -> u8 {


    0
}

define_kernel_function!(KernelFunctionName::device_init, -> u8, rpi3_init);
