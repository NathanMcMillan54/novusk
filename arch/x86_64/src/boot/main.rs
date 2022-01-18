use core::ptr::write_volatile;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    write_volatile(0xb8000 as *mut u8, b'S');
    write_volatile(0xb8002 as *mut u8, b't');
    write_volatile(0xb8004 as *mut u8, b'a');
    write_volatile(0xb8006 as *mut u8, b'r');
    write_volatile(0xb8008 as *mut u8, b't');
    write_volatile(0xb800a as *mut u8, b'i');
    write_volatile(0xb800c as *mut u8, b'n');
    write_volatile(0xb800e as *mut u8, b'g');

    panic!("x86_64 kernel init");
}
