global_asm!(include_str!("boot64.S"));

use arm::mm::linker_mem::{clear_bss_se};
use core::ptr::write_volatile;

#[no_mangle]
pub unsafe extern "C" fn aarch64_boot_setup() -> ! {
    clear_bss_se();

    for chars in "Starting kernel...\n".chars() {
        write_volatile(0x3F20_0000 as *mut u8, chars as u8);
    }

    loop { asm!("wfe"); }
}
