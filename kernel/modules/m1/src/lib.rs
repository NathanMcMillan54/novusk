#![no_std]

extern "C" {
    fn application_type() -> &'static str;
    fn main_color() -> &'static str;
    fn set_userspace_info(atype: &'static str, color: &'static str);
}

static mut APPLICATION_TYPE: &'static str = "";
static mut MAIN_COLOR: &'static str = "";

pub unsafe fn m1_init() {
    APPLICATION_TYPE = application_type();
    MAIN_COLOR = main_color();
}

pub unsafe fn m1_exit() {
    set_userspace_info(APPLICATION_TYPE, MAIN_COLOR);
}
