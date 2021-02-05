#![feature(alloc_error_handler, asm, global_asm)]
#![no_std]
#![no_main]

mod allocator;
mod keyboard;
mod panic;

// Novusk crates
extern crate arch;
extern crate drivers;
extern crate os;

#[macro_use]
extern crate novusk_lib;

// External crates
extern crate pc_keyboard;

#[no_mangle]
pub extern "C" fn kernel_init() -> ! {
    kprint!("Kernel init\n");
    unsafe { keyboard::keyboard_init(); }
    kinfo!("Keyboard initialized\n");
    unsafe { kernel_main() }
}

unsafe fn kernel_main() -> ! {
    kprint!("Kernel main\n");
    kinfo!("Starting userspace processes\n");
    kprint!("   Starting os...\n");
    os::main();
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    arch::x86::include::asm::hlt()
}
