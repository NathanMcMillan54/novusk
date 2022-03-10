use crate::boot::main::arm_boot_main;

#[cfg(feature = "cortex_m")]
mod cm_start;

#[no_mangle]
pub unsafe extern "C" fn start_boot() -> ! {
    arm_boot_main();

    panic!("Boot function should not return");
}
