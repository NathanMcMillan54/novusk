#[cfg(feature = "grub")]
global_asm!(include_str!("grub.S"));

use crate::drivers::grub;

pub unsafe fn bootloader_init() {
    #[cfg(feature = "grub")]
    grub::grub_init();
}
