#![no_std]

use libnu::ktypes::ApplicationType;

extern "C" {
    fn application_type() -> ApplicationType;
    fn main_color() -> &'static str;
    fn set_userspace_info(atype: ApplicationType, color: &'static str);
}

static mut APPLICATION_TYPE: ApplicationType = ApplicationType::None;
static mut MAIN_COLOR: &'static str = "";

pub unsafe fn m1_init() {
    APPLICATION_TYPE = application_type();
    MAIN_COLOR = main_color();
}

pub unsafe fn m1_exit() {
    set_userspace_info(APPLICATION_TYPE, MAIN_COLOR);
}
