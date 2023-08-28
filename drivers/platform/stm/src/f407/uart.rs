use stm32f4xx_hal::pac::USART1;
use core::fmt::Write;
use novuskinc::console::KernelConsoleDriver;
use novuskinc::drivers::{add_driver, Driver, DriverResult};
use novuskinc::drivers::names::SERIAL;
use novuskinc::fb::FrameBufferGraphics;
use novuskinc::keyboard::KeyboardInput;
use novuskinc::led::Led;
use novuskinc::prelude::{Serial, Storage, Timer};
use stm32f4xx_hal::pac::Peripherals;
use stm32f4xx_hal::prelude::{*};
use stm32f4xx_hal::uart::{Rx, Tx};

pub struct F4Uart {
    pub uart_tx: Option<Tx<USART1>>,
    pub uart_rx: Option<Rx<USART1>>,
}

impl F4Uart {
    pub fn tx_init(&mut self) {
        let mut peripherals = Peripherals::take().unwrap();
        let mut gpiob = peripherals.GPIOB.split();
        let mut rcc = peripherals.RCC.constrain();
        let clocks = rcc.cfgr.use_hse(25.MHz()).freeze();

        let tx_pin = gpiob.pb6;

        let mut tx: Tx<USART1> = peripherals.USART1.tx(tx_pin, 9600.bps(), &clocks).unwrap();

        tx.write_str("test");
        self.uart_tx.replace(tx);
    }

    pub fn rx_init(&mut self) {
        let mut peripherals = Peripherals::take().unwrap();
        let mut gpiob = peripherals.GPIOB.split();
        let mut rcc = peripherals.RCC.constrain();
        let clocks = rcc.cfgr.use_hse(25.MHz()).freeze();

        let rx_pin = gpiob.pb7;

        let mut rx: Rx<USART1> = peripherals.USART1.rx(rx_pin, 9600.bps(), &clocks).unwrap();

        self.uart_rx.replace(rx);
    }
}

impl KernelConsoleDriver for F4Uart {}

impl FrameBufferGraphics for F4Uart {}

impl KeyboardInput for F4Uart {}

impl Storage for F4Uart {}

impl Serial for F4Uart {
    fn write(&self, byte: u8) {

    }
}

impl Write for F4Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        todo!()
    }
}

impl Led for F4Uart {}

impl Timer for F4Uart {}

impl Driver for F4Uart {
    fn driver_name(&self) -> &'static str {
        "STM32F4 UART Driver"
    }

    fn init(&mut self) -> DriverResult {
        self.tx_init();
        self.rx_init();
        
        return if self.uart_tx.is_none() || self.uart_rx.is_none() {
            Err("Failed to initialize UART")
        } else { Ok(()) }
    }

    fn name(&self) -> &'static str {
        SERIAL
    }
}

pub unsafe fn add_uart_driver() {
    static mut F4UART: F4Uart = F4Uart {
        uart_tx: None,
        uart_rx: None
    };

    add_driver(&mut F4UART as &mut dyn Driver);
}

