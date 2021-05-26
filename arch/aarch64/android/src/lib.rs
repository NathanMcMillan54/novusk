#![no_std]

extern crate arm_lib;

use core::panic::PanicInfo;

extern "C" {
    fn aarch64_android_kernel_init();
    fn custom_android_panic(panic: &PanicInfo) -> !;
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    aarch64_android_kernel_init();
    arm_lib::include::asm::wfe()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { custom_android_panic(_info) }
}
