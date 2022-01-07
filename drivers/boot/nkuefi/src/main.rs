#![no_std]
#![no_main]
#![feature(asm)]

extern crate rlibc;
#[macro_use] extern crate uefi_services;

use core::fmt::{Arguments, Write};
use uefi::prelude::{Boot, Handle, SystemTable};
use uefi_services::system_table;

pub mod gop;

use gop::GopWriter;

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
    for _ in 0..9000000 { unsafe { asm!("nop"); } }

    let gop = GopWriter::new();
    gop.init(system_table.boot_services());

    gop.clear_screen(system_table.boot_services(), (222, 165, 132));

    _efi_print(format_args!("{}", "GOP initialized"));

    panic!("Bootloader ended");
}
