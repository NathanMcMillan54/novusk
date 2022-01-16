#![no_std]
#![no_main]
#![feature(asm)]
#![allow(warnings)]

#[macro_use] extern crate alloc;
extern crate rlibc;
#[macro_use] extern crate uefi_services;

use core::fmt::{Arguments, Write};
use uefi::prelude::{Boot, Handle, SystemTable};
use uefi_services::system_table;

pub mod arch;
pub mod gop;
pub mod sfs;

use gop::GopWriter;
use crate::sfs::SfsInterface;

pub const KERNEL_BIN_PATH: &'static str = "/boot/efi/kernel.elf";

pub(crate) fn _efi_print(fmt: Arguments) {
    unsafe { writeln!(*system_table().as_mut().stdout(), "{}", fmt); }
}

#[no_mangle]
pub extern "C" fn efi_main(image: Handle, mut system_table: SystemTable<Boot>) -> ! {
    uefi_services::init(&mut system_table);

    let stdout = system_table.stdout();

    stdout.clear();

    _efi_print(format_args!("{}", "NKUEFI successfully booted\n"));

    // Delay for a bit
    for _ in 0..99000000 { unsafe { asm!("nop"); } }

    let gop = GopWriter::new();
    gop.init(system_table.boot_services());

    gop.clear_screen(system_table.boot_services(), (222, 165, 132));

    _efi_print(format_args!("{}", "GOP initialized"));

    let sfs = SfsInterface::new();
    let mut root = sfs.get_sfs(system_table.boot_services()).open_volume().unwrap().unwrap();

    if root.read_entry(&mut vec![0; 128]).is_err() {
        panic!("Failed to read root directory");
    }

    _efi_print(format_args!("{}", "Read root directory from SFS"));
    _efi_print(format_args!("{}", "Loading kernel..."));

    arch::start_loading_kernel(system_table.boot_services());

    panic!("Bootloader ended");
}
