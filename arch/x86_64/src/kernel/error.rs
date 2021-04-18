use super::vga::WRITER;
use crate::include::kernel::die;

pub unsafe fn test_error(test_name: &str, end: bool) {
    // If the format text test failed it would make no sense to write this all as formatted text
    // So for now it'll be printed one part at a time
    WRITER.lock().write_string(test_name); WRITER.lock().write_string(" failed\n");
    if end == true {
        WRITER.lock().write_string("   Test was for important kernel feature\n   Dying in 3 seconds\n");
        // sleep(3);
        die();
    } else { return; }
}
