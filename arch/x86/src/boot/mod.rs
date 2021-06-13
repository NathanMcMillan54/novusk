global_asm!(include_str!("header.S"));

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in b"Starting kernel...".iter().enumerate() {
        *vga_buffer.offset(i as isize * 2) = byte;
        *vga_buffer.offset(i as isize * 2 + 1) = 0xf;
    }

    loop {  }
}
