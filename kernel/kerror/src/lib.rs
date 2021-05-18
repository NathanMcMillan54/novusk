#![no_std]

extern "C" {
    fn info_error() -> &'static str;
}

pub unsafe fn kerror() {
    info_error();
}
