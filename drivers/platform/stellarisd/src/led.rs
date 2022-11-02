use tm4c123x_hal::Peripherals;
use tm4c123x_hal::gpio::GpioExt;
use tm4c123x_hal::sysctl::SysctlExt;
use novuskinc::kernel::types::KernelFunctionName;

#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub enum LedColor {
    Red,
    Green,
    Blue,
}

pub struct StellarisLed {
    pub color: LedColor,
}

impl StellarisLed {
    pub fn new(led_color: LedColor) -> Self {
        return StellarisLed { color: led_color };
    }

    pub fn blink(&mut self, sleep_time: usize) {
        let mut peripherals = Peripherals::take().unwrap();
        let mut stsctl = peripherals.SYSCTL.constrain();
        let pins = peripherals.GPIO_PORTF.split(&stsctl.power_control);

        let red_led = pins.pf1.into_push_pull_output();
        let blue_led = pins.pf2.into_push_pull_output();
        let green_led = pins.pf3.into_push_pull_output();
    }
}

fn stellaris_blink(sleep: usize) -> u8 {
    let mut led = StellarisLed::new(LedColor::Red);
    led.blink(sleep);

    0
}

define_kernel_function!(KernelFunctionName::led_blink, sleep: usize, -> u8, stellaris_blink);
