#![feature(alloc_error_handler, asm, global_asm)]
#![no_std]
#![no_main]

mod allocator;
mod end;
mod keyboard;
mod panic;
mod time;
mod userspace;

// Novusk crates
extern crate arch;
use arch::ARCH;
extern crate drivers;
extern crate os;

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
    unsafe { kernel_main(); }
}

unsafe fn kernel_main() -> ! {
    kprint!("Kernel main\n");
    kinfo!("Starting userspace processes\n");
    userspace::userspace_init()
}

pub unsafe fn end_kernel() -> ! {
    kinfo!("End of kernel\n");
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    end::x86_end_kernel();

    #[cfg(any(target_arch = "aarch64"))]
    end::aarch64_end_kernel()
}
