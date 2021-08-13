use super::{BUFFER_HEIGHT, BUFFER_WIDTH, VGA_ADDRESS};

pub unsafe fn vga_init(width: usize, height: usize, address: usize) {
    assert_eq!(width, BUFFER_WIDTH);
    assert_eq!(height, BUFFER_HEIGHT);
    assert_eq!(address, VGA_ADDRESS);
}
