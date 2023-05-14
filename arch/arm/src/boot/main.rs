use core::fmt::Write;
use core::panic::PanicInfo;
use core::ptr::write_volatile;
use novuskinc::drivers::get_driver;
use novuskinc::drivers::names::SIMPLE_UART;
use novuskinc::kernel::start_kernel;
use super::setup::ArmBootSetup;

unsafe fn arm_boot_setup() {
    let boot_setup = ArmBootSetup::new();
    boot_setup.setup();
}

unsafe fn arm_main() {
    arm_boot_setup();
    write_volatile(0x4000_C000 as *mut u8, b'd');
}

#[cfg(not(feature = "cortex_m"))]
#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    arm_main();
    panic!("Kernel ended");
}

#[cfg(feature = "cortex_m")]
#[entry]
unsafe fn cm_boot_main() -> ! {
    arm_main();
    panic!("Kernel ended");
}

#[panic_handler]
unsafe fn _panic(info: &PanicInfo) -> ! {
    if get_driver(SIMPLE_UART).is_some() {
        unsafe { write_volatile(0x4000_C000 as *mut u8, b'y') };
    } else { write_volatile(0x4000_C000 as *mut u8, b'n'); }

            get_driver(SIMPLE_UART).unwrap().write(b'p');
    loop { }
}
