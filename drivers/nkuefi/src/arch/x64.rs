use alloc::string::ToString;
use core::borrow::BorrowMut;
use core::ffi::CStr;
use core::fmt::{Arguments, Write};
use crate::{KERNEL_BIN_PATH, sfs::SfsInterface, st_write};
use uefi::prelude::{Boot, BootServices, cstr16};
use uefi::proto::media::file::{Directory, File, FileAttribute, FileInfo, FileMode, FileSystemVolumeLabel, RegularFile};
use uefi::{Char16, CStr16, Handle, ResultExt};
use uefi::proto::media::file::FileType::Dir;
use uefi::table::boot::{AllocateType, MemoryType};
use uefi::table::SystemTable;
use uefi_services::println;
use x86_64::VirtAddr;
use xmas_elf::ElfFile;
use crate::exit::exit_bootservices;

unsafe fn load_kernel(bt: &BootServices) -> (ElfFile, u64, u16) {
    let mut info_buffer = [0u8; 0x100];

    let sfsi = SfsInterface::new();
    let mut sfs = sfsi.get_sfs(bt);

    let mut root = &mut sfs.open_volume().unwrap();
    let mut efi = &mut Directory::new(root.open(cstr16!("efi"), FileMode::Read, FileAttribute::empty()).unwrap());
    let mut boot = &mut Directory::new(efi.open(cstr16!("boot"), FileMode::Read, FileAttribute::empty()).unwrap());
    let kernel_file = boot.open(cstr16!("kernel.elf"), FileMode::Read, FileAttribute::SYSTEM).unwrap();

    let mut kernel_handle = RegularFile::new(kernel_file);

    let info_mem = kernel_handle.get_info::<FileInfo>(&mut info_buffer).unwrap();
    st_write(format_args!("{}{}\n", "Kernel file name: ", info_mem.file_name().to_string()));

    let pages = info_mem.file_size() / 0x1000 + 1;
    let memory_start = bt.allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, pages as usize).unwrap();

    let buffer = core::slice::from_raw_parts_mut(memory_start as *mut u8, (pages * 0x1000) as usize);
    let len: usize = kernel_handle.read(buffer).unwrap();
    let kernel_elf = ElfFile::new(&buffer[..len]).expect("Failed to parse kernel ELF");
    let kernel_entry_addr = kernel_elf.header.pt2.entry_point();
    let kernel_entry_size = kernel_elf.header.pt2.header_size();

    st_write(format_args!("{}{}\n", "kernel.elf Entry address: ", kernel_entry_addr));

    return (kernel_elf, kernel_entry_addr, kernel_entry_size);
}

pub fn start_loading_kernel(image: Handle, mut st: SystemTable<Boot>) -> ! {
    let (elf, kernel_addr, kernel_size) = unsafe { load_kernel(st.boot_services()) };

    st_write(format_args!("{}\n", "Starting kernel...\n"));

    exit_bootservices(image, st);

    unsafe {
        let kernel_entry: unsafe extern "C" fn() = core::mem::transmute(kernel_addr + (kernel_size as u64 * 0x1000));

        (kernel_entry)();
    }

    loop {  }
}
