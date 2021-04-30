use uefi::{Handle, Status};
use uefi::table::{Boot, SystemTable};

#[no_mangle]
pub unsafe extern "C" fn efi_main(image: Handle, st: SystemTable<Boot>) {
    loop {  }
}

