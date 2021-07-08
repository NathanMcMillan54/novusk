global_asm!(include_str!("boot64.S"));

use arm::mm::linker_mem::{clear_bss_se};

#[no_mangle]
pub unsafe extern "C" fn aarch64_boot_setup() -> ! {
    clear_bss_se();

    loop { asm!("wfe"); }
}
