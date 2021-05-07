global_asm!(include_str!("power/shutdown.S"));
global_asm!(include_str!("init.S"));
use super::power::shutdown;
use super::io::VirtWriter;
use crate::kernel::time::sleep::sleepm;

#[no_mangle]
pub unsafe extern "C" fn virt_init() {
    let mut writer = VirtWriter;
    writer.write_string("Starting Novusk on Aarch64 Qemu Virt\n");
    sleepm(1000);
    writer.write_string("1 second has passed\n");
}
