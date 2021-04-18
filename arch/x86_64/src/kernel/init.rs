use super::vga::WRITER;

pub unsafe fn init() {
    WRITER.lock().write_str("Starting kernel...\n");
}
