#![no_std]
#![no_main]
#![feature(abi_efiapi)]

#[macro_use] extern crate alloc;
#[macro_use] extern crate uefi;

use core::fmt::{Arguments, Write};
use core::panic::PanicInfo;
use uefi::Handle;
use uefi::prelude::{Boot, SystemTable};
use uefi_services::system_table;
use crate::exit::exit_bootservices;
use crate::sfs::SfsInterface;

pub mod arch;
pub mod exit;
pub mod sfs;

pub const KERNEL_BIN_PATH: &str = "kernel.elf";

pub(crate) fn st_write(fmt: Arguments) {
    unsafe {
        let mut st = system_table().as_mut();
        st.stdout().write_fmt(fmt).unwrap();
    }
}

#[no_mangle]
pub extern "efiapi" fn efi_main(image: Handle, mut st: SystemTable<Boot>) -> ! {
    uefi_services::init(&mut st);

    st.stdout().clear().unwrap();

    st_write(format_args!("{}", "Starting bootloader...\n"));

    let sfs = SfsInterface::new();
    let mut root = sfs.get_sfs(st.boot_services()).open_volume().unwrap();

    if root.read_entry(&mut vec![0; 127]).is_err() {
        panic!("Failed to read root directory");
    } else { st_write(format_args!("{}", "Got root directory\n")); }


    let mut buffer = vec![0; 128];

    loop {
        let file_info = match root.read_entry(&mut buffer) {
            Ok(completion) => {
                if let Some(info) = completion {
                    info
                } else {
                    // We've reached the end of the directory
                    break;
                }
            }
            Err(error) => {
                // Buffer is not big enough, allocate a bigger one and try again.
                let min_size = error.data().unwrap();
                buffer.resize(min_size, 0);
                continue;
            }
        };
    }

    arch::start_loading_kernel(image, st);
}
