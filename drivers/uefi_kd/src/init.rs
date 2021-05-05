use core::fmt::Write;
use uefi::Handle;
use uefi::table::{SystemTable, Boot};
use uefi::proto::console::text::{Input, Output};

extern "C" {
    fn cmdline_init(stdout: *mut Output);
    fn keyboard_init(stdin: *mut Input);
}

pub unsafe fn uefi_init(handler: Handle, system_table: SystemTable<Boot>) {
    cmdline_init(system_table.stdout());
    keyboard_init(system_table.stdin());
}
