#![feature(asm, global_asm)]
#![no_std]
#![no_main]

mod panic;

extern crate arch;
extern crate os;

#[macro_use]
extern crate novusk_lib;

#[no_mangle]
pub extern "C" fn kernel_init() -> ! {
    kprint!("Kernel init\n");
    unsafe { kernel_main() }
}

unsafe fn kernel_main() -> ! {
    kprint!("Kernel main\n");
    kinfo!("Starting userspace processes\n");
    os::main();
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    arch::x86::include::asm::hlt()
}
