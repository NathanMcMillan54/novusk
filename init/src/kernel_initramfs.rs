use libn::libnu::io::fs::{IFile, FileMode, FileAttribute};
use uefi::proto::media::file::File;

#[no_mangle]
pub unsafe extern "C" fn kernel_initramfs_main() {
    let mut root = IFile.get();

    // Write memory map to /temp/kernm.txt
    let kernm = root.open("temp/kernm.txt", FileMode::CreateReadWrite, FileAttribute::SYSTEM);
}
