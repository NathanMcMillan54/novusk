pub mod write;
use write::SERIAL_PORT;

pub(crate) unsafe fn init() {
    SERIAL_PORT.init();
}
