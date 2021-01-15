#![feature(asm, global_asm)]
#![no_std]
#![no_main]

mod panic;

extern crate arch;

#[no_mangle]
pub extern "C" fn kernel_init() -> ! {
    unsafe { kernel_main() }
}

unsafe fn kernel_main() -> ! {
    loop {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        asm!("hlt");
    }
}
