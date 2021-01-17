#![feature(asm, global_asm)]
#![no_std]
#![no_main]

mod kinfo;
mod kprint;
mod panic;

extern crate arch;
extern crate os;

#[no_mangle]
pub extern "C" fn kernel_init() -> ! {
    kprint!("Kernel init\n");
    unsafe { kernel_main() }
}

unsafe fn kernel_main() -> ! {
    kprint!("Kernel main\n");
    kinfo!("Starting userspace processes\n");
    os::main();
    loop {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        asm!("hlt");
    }
}
