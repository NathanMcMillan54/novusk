use core::fmt::Write;
use bootloader::BootInfo;
use core::ptr::write_volatile;
use x86_64::instructions::hlt;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    let mut addr = 0xb8000 as *mut u8;
    for b in b"Made it!" {
        write_volatile(addr, *b);
        addr = addr.offset(2);
    }

    loop {  }
}
