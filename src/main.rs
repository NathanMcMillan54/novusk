#![feature(asm, global_asm)]
#![no_std]
#![no_main]

mod panic;

extern crate arch;

const TEXT: &[u8] = b"Novusk";

#[no_mangle]
pub extern "C" fn kernel_init() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in TEXT.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 15;
        }
    }
    unsafe { kernel_main() }
}

unsafe fn kernel_main() -> ! {
    loop {
        asm!("hlt");
    }
}
