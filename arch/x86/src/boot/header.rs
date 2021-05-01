use super::{cpu, main::bmain};
use super::boot::{die, UefiBootArgs};
use uefi::{Handle};
use uefi::table::{Boot, SystemTable};
use crate::drivers::uefi_init;

#[no_mangle]
pub unsafe extern "C" fn efi_main(image: Handle, system_table: SystemTable<Boot>) -> ! {
    if !cpu::check_arch() {
        die();
    }
    uefi_init(image, system_table);
    bmain()
}
