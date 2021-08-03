use super::{device, uart::Uart};
use hifive1::clock;
use hifive1::hal::clock::Clocks;
use hifive1::hal::device::{DevicePeripherals, DeviceResources};
use hifive1::hal::time::U32Ext;

pub struct Board;

impl Board {
    pub fn peripherals(&mut self) -> DevicePeripherals {
        let dr = DeviceResources::take().unwrap();
        let p = dr.peripherals;

        return p;
    }

    pub fn clocks(&mut self) -> Clocks {
        let p = self.peripherals();
        let mut clocks = clock::configure(p.PRCI, p.AONCLK, 320.mhz().into());

        return clocks;
    }
}

impl device::Device for Board {
    fn device_init(&mut self) {
        self.io_init();
    }

    fn io_init(&mut self) {
        // This probably doesn't work but all sifive boards are micro controllers and no one will
        // see this
        unsafe { Uart::new(0x1001_3000).uart_init(); }
    }

    fn name(&mut self) -> &'static str {
        return "HiFive";
    }
}
