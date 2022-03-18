// All these start functions are called from the cortex-m-rt library
use crate::boot::main::arm_boot_main;
use cortex_m::Peripherals;

extern "C" {
    fn boot_die() -> !;
}

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    hprintln!("Starting kernel on Cortex M CPU...");

    arm_boot_main();

    boot_die();
}

#[no_mangle]
pub unsafe extern "C" fn __pre_init() {
    extern "C" {
        static mut __sbss: u64;
        static mut __ebss: u64;
    }


    if __ebss != 0 {
        hprintln!("BSS end = {}", __ebss);
        panic!("BSS end does not equal 0");
    }

    if Peripherals::take().is_none() {
        hprintln!("Can't find CPU peripherals");
        panic!("Can't find CPU peripherals, the buid target of Novusk image might be running on  an incompatible CPU");
    }
}
