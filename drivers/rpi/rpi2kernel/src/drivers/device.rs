use novuskinc::kernel::types::KernelFunctionName;
use rpi::Rpi2;

#[no_mangle]
pub extern "C" fn rpi2_init() -> u8 {
    let mut pi = Rpi2::new();
    pi.init();

    return 0;
}

define_kernel_function!(KernelFunctionName::device_init, -> u8, rpi2_init);
