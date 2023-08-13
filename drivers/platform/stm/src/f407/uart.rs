use stm32f4xx_hal::pac::USART1;
use core::fmt::Write;
use stm32f4xx_hal::pac::Peripherals;
use stm32f4xx_hal::prelude::{*};
use stm32f4xx_hal::uart::{Rx, Tx};

lazy_static::lazy_static! {
    static ref F407UART: spin::Mutex<F407Uart> = spin::Mutex::new(F407Uart {
        uart_tx: None,
        uart_rx: None,
    });
}

pub struct F407Uart {
    pub uart_tx: Option<Tx<USART1>>,
    pub uart_rx: Option<Rx<USART1>>,
}

impl F407Uart {
    pub fn tx_init(&mut self) {
        let mut peripherals = Peripherals::take().unwrap();
        let mut gpioa = peripherals.GPIOA.split();
        let mut rcc = peripherals.RCC.constrain();
        let clocks = rcc.cfgr.use_hse(25.MHz()).freeze();

        let tx_pin = gpioa.pa9;

        let mut tx: Tx<USART1> = peripherals.USART1.tx(tx_pin, 9600.bps(), &clocks).unwrap();

        self.uart_tx.replace(tx);
    }

    pub fn rx_init(&mut self) {
        let mut peripherals = Peripherals::take().unwrap();
        let mut gpioa = peripherals.GPIOA.split();
        let mut rcc = peripherals.RCC.constrain();
        let clocks = rcc.cfgr.use_hse(25.MHz()).freeze();

        let rx_pin = gpioa.pa10;

        let mut rx: Rx<USART1> = peripherals.USART1.rx(rx_pin, 9600.bps(), &clocks).unwrap();

        self.uart_rx.replace(rx);
    }
}

pub fn uart_init() {
    F407UART.lock().tx_init();
    //F407UART.lock().rx_init();

    writeln!(F407UART.lock().uart_tx.as_mut().unwrap(), "this is a test");
}

