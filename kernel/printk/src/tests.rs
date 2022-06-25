use core::fmt::Write;
use novuskinc::console::KernelConsoleDriver;
use novuskinc::drivers::{Driver, DriverResult, manager::DeviceDriverManager};
use novuskinc::fb::FrameBufferGraphics;
use novuskinc::keyboard::KeyboardInput;
use crate::{Printk, can_printk_work, init::console_init};

// For handling "device" (test) driver[s]
#[no_mangle]
pub(crate) static mut DEVICE_DRIVERS: DeviceDriverManager = DeviceDriverManager {
    drivers: None,
};

// For handling printk
#[no_mangle]
pub(crate) static mut PRINTK: Printk = Printk {
    init: false,
    console_driver: None
};

struct TestConsole;

impl Write for TestConsole {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        Ok(())
    }
}

impl KernelConsoleDriver for TestConsole {
    fn write_character(&self, c: char, x: u16, y: u16) {

    }
}

impl FrameBufferGraphics for TestConsole {}

impl KeyboardInput for TestConsole {}

impl Driver for TestConsole {
    fn driver_name(&self) -> &'static str {
        return "Test Console";
    }

    fn name(&self) -> &'static str {
        return "Console Driver";
    }

    fn init(&self) -> Option<DriverResult> {
        Some(Ok(()))
    }
}

pub fn console_init_test() {
    unsafe {
        DEVICE_DRIVERS.add_driver(&TestConsole as &'static dyn Driver);

        console_init();

        assert_eq!(true, can_printk_work())
    }
}
