use crate::drivers::virt::power::shutdown;
global_asm!(include_str!("power/shutdown.S"));
global_asm!(include_str!("start.S"));

#[no_mangle]
pub unsafe extern "C" fn virt_init() {
    // TODO: Initialize virt "board"
}
