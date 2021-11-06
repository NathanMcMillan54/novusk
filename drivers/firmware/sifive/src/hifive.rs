use device::Device;
use crate::HiFiveBoard;

use hifive1::clock;
use hifive1::stdout;
use hifive1::hal::DeviceResources;
use hifive1::hal::time::U32Ext;
use hifive1::hal::clock::Clocks;

impl HiFiveBoard {
    fn get_clocks(&self) -> Clocks {
        let dev_res = DeviceResources::take().unwrap();
        let peripherals = dev_res.peripherals;

        let clocks: Clocks = clock::configure(peripherals.PRCI, peripherals.AONCLK, 320.mhz().into());

        return clocks;
    }
}

impl Device for HiFiveBoard {
    fn name(&self) -> &'static str {
        return "HiFive";
    }

    fn serial_io_init(&self) {
        let dev_res = DeviceResources::take().unwrap();
        let peripherals = dev_res.peripherals;
        let pins = dev_res.pins;

        stdout::configure(peripherals.UART0, pin!(pins, uart0_tx), pin!(pins, uart0_rx), 115_200.bps(), self.get_clocks());
    }

    fn time_init(&self) {
        self.get_clocks();
    }

    fn gpio_init(&self) {

    }
}
