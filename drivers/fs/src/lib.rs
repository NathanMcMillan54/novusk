#![no_std]

#[macro_use] extern crate kerror;

pub static mut INIT: bool = false;
pub static mut FS: &'static str = "";

#[no_mangle]
pub unsafe extern "C" fn fs_init() {
    if FS == "" && INIT == false {
        kerror!("Fs never set or initialized, will not initialialize fs");
        FS = "none"
    } else if FS == "" && INIT == true {
        kerror!("Fs never set but was initialized, you should set an fs");
        FS = "unknown";
    }

    if FS == "FAT" && INIT == false {
        fat::fat_init();
    }
}

#[no_mangle]
pub unsafe extern "C" fn set_fs(fs: &'static str) {
    FS = fs;
}

#[no_mangle]
pub unsafe extern "C" fn set_fs_init() {
    INIT = true;
}

#[no_mangle]
pub unsafe extern "C" fn fs_name() -> &'static str {
    return FS;
}
