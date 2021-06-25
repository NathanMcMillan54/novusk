// TODO: When the x86 kernel isn't as important and C FFI becomes a priority, link the OS Dev
// example of VGA buffer to the ix86 kernel

extern "C" {
    pub fn vga_ix86_init(width: i32, height: i32, address: usize);
    pub fn vga_write_char(chars: &i8);
}
