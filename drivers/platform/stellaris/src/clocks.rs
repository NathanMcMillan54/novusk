use tm4c123x_hal::Peripherals;
use tm4c123x_hal::sysctl::{CrystalFrequency, Oscillator, PllOutputFrequency, SystemClock, SysctlExt, Sysctl};

pub fn setup_sys_clocks() {
    let dev_peripherals = unsafe { Peripherals::steal() };
    let mut sctl = dev_peripherals.SYSCTL.constrain();

    sctl.clock_setup.oscillator = Oscillator::Main(
        CrystalFrequency::_16mhz,
        SystemClock::UsePll(PllOutputFrequency::_16_67mhz),
    );
}