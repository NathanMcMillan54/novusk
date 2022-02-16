use crate::boot::main::arm_boot_main;

#[cfg(target_arch = "arm")]
#[cfg(feature = "cortex_m")]
mod cm_boot32;

#[no_mangle]
pub unsafe extern "C" fn start_boot() -> ! {
    arm_boot_main();

    panic!("Boot function should not return");
}
