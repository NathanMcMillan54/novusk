use core::{arch::{asm}, fmt::Write};
use multiboot2::{BootInformation, BootInformationHeader};
use crate::boot::video::EarlyVga;


#[no_mangle]
pub unsafe extern "C" fn _bootstart(magic_n: u32, addr: u32) -> ! {
    let mut early_vga = EarlyVga::new();
    early_vga.write_fmt(format_args!("{}", "Starting...\n")).unwrap();

    let mb_info = BootInformation::load(addr as *const BootInformationHeader);

    panic!()
}
