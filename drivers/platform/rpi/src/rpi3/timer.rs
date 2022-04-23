use asminc::aarch64::io::*;
use novuskinc::core::names::CoreFunctionNames;
use crate::SOC_INFO;

const TIMER_CLO: isize = 0x4;
const TIMER_CHI: isize = 0x8;
const TIMER_C0: isize = 0xC;
const TIMER_C1: isize = 0x10;
const TIMER_C2: isize = 0x14;
const TIMER_C3: isize = 0x18;
const TIMER_CS_M0: isize = 1 << 0;
const TIMER_CS_M1: isize = 1 << 1;
const TIMER_CS_M2: isize = 1 << 2;
const TIMER_CS_M3: isize = 1 << 3;

const INTERVAL: u32 = 20000;
static mut TIMER_VALUE: u32 = 0;

unsafe fn rpi3_timer_init(_n: ()) -> () {
    let mut timer_addr = 0x0 as *mut u8;

    if SOC_INFO.name == "Broadcom BCM2837" {
        timer_addr = SOC_INFO.addresses[2].1; // Should be Timer CS Address, use get() when panic handlers are working properly
    }

    TIMER_VALUE = inb(0x3F00_3004);
    TIMER_VALUE += INTERVAL;
    outb(0x3F00_3010, TIMER_VALUE as u16);
}

define_core_function!(CoreFunctionNames::device_timer_init, _n: (), -> (), rpi3_timer_init);

aarch64_interrupt!(SysTimer1, {
    TIMER_VALUE += INTERVAL;
    outb(0x3F00_3010, TIMER_VALUE as u16);
});
