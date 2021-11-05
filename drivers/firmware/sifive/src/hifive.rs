use device::Device;
use crate::HiFiveBoard;

impl Device for HiFiveBoard {
    fn name(&self) -> &'static str {
        return "HiFive";
    }

    fn serial_io_init(&self) {

    }

    fn gpio_init(&self) {

    }
}
