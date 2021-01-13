#![feature(asm)]
#![no_std]
#![no_main]

mod panic;

extern crate arch;

#[no_mangle]
pub extern "C" fn initStart() -> ! {
    unsafe { kernel_init() }
}

unsafe fn kernel_init() -> ! {
    kernel_main()
}

unsafe fn kernel_main() -> ! {
    loop {
        asm!("hlt");
    }
}
