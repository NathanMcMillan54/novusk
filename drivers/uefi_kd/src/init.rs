use core::fmt::Write;
use uefi::Handle;
use uefi::proto::console::text::{Input, Output};
use uefi::table::{Boot, SystemTable, Revision};
use crate::{UEFI_MAJOR_VERSION, UEFI_MINOR_VERSION};

extern "C" {
    fn cmdline_init(st: SystemTable<Boot>);
}

pub unsafe fn uefi_init(handler: Handle, system_table: SystemTable<Boot>) {
    version_init(system_table.uefi_revision());
    cmdline_init(system_table);
}

unsafe fn version_init(version: uefi::table::Revision) {
    let major_version = version.major();
    let minor_version = version.minor();

    assert!(major_version >= 2, "UEFI version is unsupported");
    assert!(minor_version >= 30, "UEFI version is old, some features might not be supported");

    UEFI_MAJOR_VERSION = major_version;
    UEFI_MINOR_VERSION = minor_version;
}
