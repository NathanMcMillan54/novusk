#![no_std]
#![feature(lang_items)]

#[cfg(not(feature = "no_lang_items"))]
#[path = "../../../lang.rs"]
pub(crate) mod lang;

pub type rint8_t = libc::c_schar;
pub type rint16_t = libc::c_short;
pub type rint32_t = libc::c_int;
#[cfg(target_pointer_width = "64")]
pub type rint64_t = libc::c_longlong;

#[no_mangle]
pub extern "C" fn __use_types_stdint_intn_h(_: rint8_t, _: rint16_t, _: rint32_t, _: rint64_t) {}
