use gpu::info::{set_gpu, set_init};

pub unsafe fn vga_init(height: usize, width: usize, address: usize) {
    // TODO: Setup vga on 0xb8000 or 0xa0000 address

    set_gpu("VGA");
    set_init(true);
}
