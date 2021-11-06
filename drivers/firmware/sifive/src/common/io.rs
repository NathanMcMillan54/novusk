use crate::{HiFiveBoard, LoFiveBoard};

use hifive1::stdout;
use hifive1::hal::DeviceResources;
use hifive1::hal::time::U32Ext;
use device::Device;
use hifive1::hal::clock::Clocks;

pub struct SiFiveIo {
    // *has same i/o setup but not for clocks/time*
    pub clocks: Clocks,
}

impl SiFiveIo {
    pub fn new(clocks: Clocks) -> Self {
        return SiFiveIo { clocks: clocks };
    }

    pub fn sifive_io_init(&self) {
        let dev_res = DeviceResources::take().unwrap();
        let peripherals = dev_res.peripherals;
        let pins = dev_res.pins;

        stdout::configure(peripherals.UART0, pin!(pins, uart0_tx), pin!(pins, uart0_rx), 115_200.bps(), self.clocks);
    }
}
