use super::{VGA_ADDRESS, VGA_ADDRESS_STR};

pub unsafe fn vga_init(width: usize, height: usize, address: usize) {
    VGA_ADDRESS = address;
    if VGA_ADDRESS == 0xb8000 {
        VGA_ADDRESS_STR = "0xb8000";
    } else if VGA_ADDRESS == 0xa0000 {
        VGA_ADDRESS_STR = "0xa0000";
    } else {
        VGA_ADDRESS_STR = "Unknown";
    }
}
