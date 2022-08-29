pub type rint8_t = libc::c_schar;
pub type rint16_t = libc::c_short;
pub type rint32_t = libc::c_int;
pub type rint64_t = libc::c_longlong;

// TODO: Come up with a better implementation for least types
pub type rint_least8_t = libc::c_schar;
pub type rint_least16_t = libc::c_short;
pub type rint_least32_t = libc::c_int;
pub type rint_least64_t = libc::c_longlong;

// TODO: Come up with a better implementation for fast types
pub type rint_fast8_t = libc::c_schar;
pub type rint_fast16_t = libc::c_short;
pub type rint_fast32_t = libc::c_int;
pub type rint_fast64_t = libc::c_longlong;

#[no_mangle]
pub extern "C" fn _use_types_stdint_intn_h(
    _: rint8_t,
    _: rint16_t,
    _: rint32_t,
    _: rint64_t,
    _: rint_fast8_t,
    _: rint_fast16_t,
    _: rint_fast32_t,
    _: rint_fast64_t,
    _: rint_least8_t,
    _: rint_least16_t,
    _: rint_least32_t,
    _: rint_least64_t,
) {
}
