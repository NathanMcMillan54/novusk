use novuskinc::core::names::CoreFunctionNames;
use crate::SOC_INFO;

const TIMER_CLO: u8 = 0x4;
const TIMER_CHI: u8 = 0x8;
const TIMER_C0: u8 = 0xC;
const TIMER_C1: u8 = 0x10;
const TIMER_C2: u8 = 0x14;
const TIMER_C3: u8 = 0x18;

const INTERVAL: i32 = 20000;
static mut TIMER_VALUE: i32 = 0;

pub unsafe fn get_timer_address() -> *mut u8 {
    let ret = SOC_INFO.get("Timer CS Address");

    if ret.is_none() {
        return 0x0 as *mut u8;
    } else { return ret.unwrap(); }
}

unsafe fn rpi3_timer_init(_n: ()) -> () {

    return;
}

define_core_function!(CoreFunctionNames::device_timer_init, _n: (), -> (), rpi3_timer_init);
