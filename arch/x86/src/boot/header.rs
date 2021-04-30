use super::main::bmain;
use uefi::{Handle};
use uefi::table::{Boot, SystemTable};

unsafe fn setup(uefi_arg1: Handle, uefi_arg2: SystemTable<Boot>) {

}

#[no_mangle]
pub unsafe extern "C" fn efi_main(image: Handle, system_table: SystemTable<Boot>) -> ! {
    setup(image, system_table);
    bmain()
}
