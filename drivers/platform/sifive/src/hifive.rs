use core::str::from_utf8_unchecked;
use device::Device;
use crate::{HiFiveBoard};

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
