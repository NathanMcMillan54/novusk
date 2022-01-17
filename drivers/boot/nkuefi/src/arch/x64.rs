use alloc::string::ToString;
use core::fmt::{Arguments, Write};
use crate::{KERNEL_BIN_PATH, SfsInterface, _efi_print};
use uefi::prelude::{Boot, BootServices};
use uefi::proto::media::file::{File, FileAttribute, FileInfo, FileMode, FileSystemVolumeLabel, RegularFile};
use uefi::{Handle, ResultExt};
use uefi::table::boot::{AllocateType, MemoryType};
use uefi::table::SystemTable;
use x86_64::VirtAddr;
use xmas_elf::ElfFile;
use crate::exit::exit_bootservices;

unsafe fn load_kernel(bt: &BootServices) -> (ElfFile, VirtAddr) {
    let mut info_buffer = [0u8; 0x100];

    let sfsi = SfsInterface::new();
    let sfs = sfsi.get_sfs(bt);

    let mut root = sfs.open_volume().unwrap().unwrap();
    let volume_label = root.get_info::<FileSystemVolumeLabel>(&mut info_buffer).unwrap().unwrap().volume_label().to_string();

    let kernel_handle = root.open(KERNEL_BIN_PATH, FileMode::Read, FileAttribute::empty()).expect_success("Couldn't open kernel binary");
    let mut kernel_handle = RegularFile::new(kernel_handle);

    let info_mem = kernel_handle.get_info::<FileInfo>(&mut info_buffer).unwrap().unwrap();
    _efi_print(format_args!("{}{}", "Kernel file name: ", info_mem.file_name().to_string()));

    let pages = info_mem.file_size() / 0x1000 + 1;
    let memory_start = bt.allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, pages as usize).expect_success("Couldn't to alloc pages");

    let buffer = core::slice::from_raw_parts_mut(memory_start as *mut u8, (pages * 0x1000) as usize);
    let len: usize = kernel_handle.read(buffer).expect_success("Couldn't read kernel file");
    let kernel_elf = ElfFile::new(buffer[..len].as_ref()).expect("Failed to parse kernel ELF");
    let kernel_entry_addr = kernel_elf.header.pt2.entry_point();

    _efi_print(format_args!("{}{}", "kernel.elf Entry address: ", kernel_entry_addr));

    return (kernel_elf, VirtAddr::new(kernel_entry_addr));
}

pub fn start_loading_kernel(image: Handle, mut st: SystemTable<Boot>) {
    let (elf, kernel_addr) = unsafe { load_kernel(st.boot_services()) };

    _efi_print(format_args!("{}", "Starting kernel..."));

    exit_bootservices(image, st);

    let kernel_entry: extern "C" fn() = unsafe { core::mem::transmute(kernel_addr.as_u64()) };

    (kernel_entry)();
}
