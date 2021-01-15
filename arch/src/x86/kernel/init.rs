fn kernel_boot_msg(arg: &[u8]) {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in arg.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 15;
        }
    }
}

pub unsafe fn x86_init() {
    kernel_boot_msg("Novusk \
 v1.0.0 \
 New Kernel \
".as_bytes());
}
