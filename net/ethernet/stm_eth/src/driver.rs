use ethernet::EthNetDriver;
use crate::StmEth;
use stm32_eth::{Eth, EthPins, hal::gpio::GpioExt, stm32::Peripherals, PhyAddress, RingEntry};
use stm32_eth::hal::rcc::RccExt;
use stm32_eth::hal::time::U32Ext;

impl EthNetDriver for StmEth {
    fn write(&mut self, net_text: &[u8]) {

    }

    fn read(&mut self, buf: u8) -> &'static [u8] {
        return b"";
    }

    fn init(&mut self) {
        let peripherals = unsafe { Peripherals::steal() };

        let gpioa = peripherals.GPIOA.split();
        let gpiob = peripherals.GPIOB.split();
        let gpioc = peripherals.GPIOC.split();
        let gpiog = peripherals.GPIOG.split();

        let eth_pins = EthPins {
            ref_clk: gpioa.pa1,
            md_io: gpioa.pa2,
            md_clk: gpioc.pc1,
            crs: gpioa.pa7,
            tx_en: gpiog.pg11,
            tx_d0: gpiog.pg13,
            tx_d1: gpiob.pb13,
            rx_d0: gpioc.pc4,
            rx_d1: gpioc.pc5,
        };

        let mut rx_ring: [RingEntry<_>; 8] = Default::default();
        let mut tx_ring: [RingEntry<_>; 2] = Default::default();

        let mut eth = Eth::new(
            peripherals.ETHERNET_MAC,
            peripherals.ETHERNET_DMA,
            &mut rx_ring,
            &mut tx_ring,
            PhyAddress::_0,
            peripherals.RCC.constrain().cfgr.sysclk(32.mhz()).hclk(32.mhz()).freeze(),
            eth_pins
        );

        if eth.is_err() {
            printk!("Error occurred while initializing ethernet");
        }
    }
}
