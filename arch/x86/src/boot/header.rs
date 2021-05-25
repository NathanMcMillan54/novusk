use super::{cpu, main::bmain};
use super::boot::{die};
use core::fmt::Write;
use uefi::{Handle, ResultExt};
use uefi::table::{Boot, SystemTable};
use uefi_services;
use uefi::proto::console::text::Output;
use crate::drivers::{print_uefi_init, uefi_init};
use crate::kernel::{printk, userspace::early};

#[no_mangle]
pub unsafe extern "C" fn efi_main(image: Handle, system_table: SystemTable<Boot>) -> ! {
    early::early_user_init();
    printk::printk_init();

    uefi_services::init(&system_table).expect_success("Couldn't initialize UEFI services");

    system_table.stdout()
        .reset(false)
        .expect_success("Failed to reset UEFI stdout");

    kinfo!("Kernel printing initialized");
    kinfo!("Early ueserspace initialized");
    kinfo!("UEFI services initialized");

    uefi_init(image, system_table);
    kinfo!("Finished UEFI drivers initialization");
    print_uefi_init();
    bmain();
}
