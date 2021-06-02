use core::fmt::Write;
use fs::set_fs;
use gop::{gop_init, gop_reinit};
use gpu::{set_gpu_init, set_gpu};
use uefi::Handle;
use uefi::prelude::BootServices;
use uefi::proto::console::text::{Input, Output};
use uefi::table::{Boot, SystemTable, Revision};
use crate::{UEFI_MAJOR_VERSION, UEFI_MINOR_VERSION};

unsafe fn version_init(version: uefi::table::Revision) {
    let major_version = version.major();
    let minor_version = version.minor();

    UEFI_MAJOR_VERSION = major_version;
    UEFI_MINOR_VERSION = minor_version;

    if major_version < 2 {
        kerror!("UEFI major version is not supported");
    } else if minor_version < 30 {
        kerror!("UEFI minor version is old but might work");
    }
}

unsafe fn gpu_init(bt: &BootServices) {
    gop_init(bt);
    set_gpu("gop");
    set_gpu_init();
}

pub unsafe fn fs_init() {
    set_fs("FAT");
}

pub unsafe fn uefi_init(handler: Handle, system_table: SystemTable<Boot>) {
    version_init(system_table.uefi_revision());
    gpu_init(system_table.boot_services());
    fs_init();
    system_table.stdin().reset(true).expect("Failed to reset stdin");
}
