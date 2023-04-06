use core::panic::PanicInfo;
use core::ptr::write_volatile;
use novuskinc::kernel::start_kernel;
use super::setup::ArmBootSetup;

unsafe fn arm_boot_setup() {
    let boot_setup = ArmBootSetup::new();
    boot_setup.setup();
}

unsafe fn arm_main() {
    arm_boot_setup();
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
fn _panic(info: &PanicInfo) -> ! {
    unsafe { write_volatile(0x4000_C000 as *mut u8, b'p'); }
    loop { }
}
