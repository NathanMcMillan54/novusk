#![no_std]

#[macro_use] extern crate alloc;

#[macro_use] extern crate kerror;
#[macro_use] extern crate kinfo;
#[macro_use] extern crate printk;

use libefi::st;
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::proto::media::file::{File, Directory};

pub unsafe fn fat_init() {
    // This means BootServices and nothing else!
    let bs = st().as_ref().boot_services();

    if let Ok(sfs) = bs.locate_protocol::<SimpleFileSystem>() {
        let sfs = sfs.expect("Cannot open `SimpleFileSystem` protocol");
        let sfs = unsafe { &mut *sfs.get() };
        let mut root = sfs.open_volume().unwrap().unwrap();
        let mut buffer = vec![0; 128];
        loop {
            let file_info = match root.read_entry(&mut buffer) {
                Ok(completion) => {
                    if let Some(info) = completion.unwrap() {
                        info
                    } else {
                        break
                    }
                },
                Err(error) => {
                    let min_size = error.data().unwrap();
                    buffer.resize(min_size, 0);
                    continue;
                },
            };
            printk!("Root directory:\n    {:?}", file_info);
        }
        root.reset_entry_readout().unwrap().unwrap();
    } else {
        kerror!("Couldn't initialize FAT Fs");
    }
}

pub unsafe fn fat_reinit() -> Directory {
    let bs = st().as_ref().boot_services();

    if let Ok(sfs) = bs.locate_protocol::<SimpleFileSystem>() {
        let sfs = sfs.expect("Cannot open `SimpleFileSystem` protocol");
        let sfs = unsafe { &mut *sfs.get() };
        let mut directory = sfs.open_volume().unwrap().unwrap();
        let mut buffer = vec![0; 128];
        loop {
            let file_info = match directory.read_entry(&mut buffer) {
                Ok(completion) => {
                    if let Some(info) = completion.unwrap() {
                        info
                    } else {
                        break
                    }
                },
                Err(error) => {
                    let min_size = error.data().unwrap();
                    buffer.resize(min_size, 0);
                    continue;
                },
            };
        }
        return directory;
    } else {
        kerror!("Couldn't reinitialize FAT Fs");
        return fat_reinit();
    }
}
