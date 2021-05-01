use uefi::Handle;
use uefi::table::{SystemTable, Boot};

use core::fmt::Write;

pub unsafe fn uefi_init(handler: Handle, sytem_table: SystemTable<Boot>) {
    let stdout = sytem_table.stdout();
    stdout.clear().unwrap().unwrap();
    writeln!(stdout, "Setting up kernel UEFI...").unwrap();
}