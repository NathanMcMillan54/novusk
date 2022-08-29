pub type ruint8_t = libc::c_uchar;
pub type ruint16_t = libc::c_ushort;
pub type ruint32_t = libc::c_uint;
pub type ruint64_t = libc::c_ulonglong;

// TODO: Come up with a better implementation for least types
pub type ruint_least8_t = libc::c_uchar;
pub type ruint_least16_t = libc::c_ushort;
pub type ruint_least32_t = libc::c_uint;
pub type ruint_least64_t = libc::c_ulonglong;

// TODO: Come up with a better implementation for fast types
pub type ruint_fast8_t = libc::c_uchar;
pub type ruint_fast16_t = libc::c_ushort;
pub type ruint_fast32_t = libc::c_uint;
pub type ruint_fast64_t = libc::c_ulonglong;

#[no_mangle]
pub extern "C" fn __use_types_stdint_uintn_h(
    _: ruint8_t,
    _: ruint16_t,
    _: ruint32_t,
    _: ruint64_t,
    _: ruint_fast8_t,
    _: ruint_fast16_t,
    _: ruint_fast32_t,
    _: ruint_fast64_t,
    _: ruint_least8_t,
    _: ruint_least16_t,
    _: ruint_least32_t,
    _: ruint_least64_t,
) {
}
