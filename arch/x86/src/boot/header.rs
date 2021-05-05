use super::{cpu, main::bmain};
use super::boot::{die};
use uefi::{Handle};
use uefi::table::{Boot, SystemTable};
use uefi_services;
use crate::drivers::uefi_init;

#[no_mangle]
pub unsafe extern "C" fn efi_main(image: Handle, system_table: SystemTable<Boot>) -> ! {
    let st = system_table;
    if !cpu::check_arch() {
        die();
    }
    uefi_services::init(&st).expect("Couldn't initialize UEFI services");
    uefi_init(image, st);
    bmain()
}
