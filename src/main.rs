#![feature(alloc_error_handler, asm, global_asm)]
#![no_std]
#![no_main]

mod allocator;
mod keyboard;
mod panic;
mod time;
mod userspace;

// Novusk crates
extern crate arch;
extern crate drivers;
extern crate os;

use arch::ARCH;

#[macro_use]
extern crate novusk_lib;

// External crates
extern crate pc_keyboard;

#[no_mangle]
pub extern "C" fn kernel_init() {
    kprint!("Kernel init\n");
    keyboard::keyboard_init();
    kinfo!("Keyboard initialized\n");
    kprint!("   Setup keyboard for {}\n", ARCH);
    time::time_reinit();
    kinfo!("Kernel time reinitialized\n");
    unsafe { kernel_main(); }
}

unsafe fn kernel_main() -> ! {
    kprint!("Kernel main\n");
    kinfo!("Starting userspace processes\n");
    userspace::userspace_init()
}

pub unsafe fn end_kernel() -> ! {
    kinfo!("End of kernel\n");
    use arch::x86::include::asm;
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    asm::hlt()
}
