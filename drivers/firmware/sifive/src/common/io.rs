use crate::{HiFiveBoard, LoFiveBoard};

use hifive1::clock;
use hifive1::hal::clock::Clocks;
use hifive1::stdout;
use hifive1::hal::DeviceResources;
use hifive1::hal::time::U32Ext;
use device::Device;

pub struct SiFiveIo {
    // *has same i/o setup but not for clocks/time*
    pub clocks: Clocks,
}

impl SiFiveIo {
    fn get_clock() -> Clocks {
        let dev_res = DeviceResources::take().unwrap();
        let peripherals = dev_res.peripherals;

        let clocks = clock::configure(peripherals.PRCI, peripherals.AONCLK, 320.mhz().into());

        return clocks;
    }

    pub fn new() -> Self {
        return SiFiveIo { clocks: SiFiveIo::get_clock() };
    }

    pub fn sifive_io_init(&self) {
        let dev_res = DeviceResources::take().unwrap();
        let peripherals = dev_res.peripherals;
        let pins = dev_res.pins;

        stdout::configure(peripherals.UART0, pin!(pins, uart0_tx), pin!(pins, uart0_rx), 115_200.bps(), self.clocks);
    }
}
