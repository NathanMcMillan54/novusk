pub type ruint8_t = libc::c_uchar;
pub type ruint16_t = libc::c_ushort;
pub type ruint32_t = libc::c_uint;
#[cfg(target_pointer_width = "64")]
pub type ruint64_t = libc::c_ulong;
#[cfg(target_pointer_width = "32")]
pub type ruint64_t = libc::c_ulonglong;

#[no_mangle]
pub extern "C" fn __use_types_stdint_uintn_h(
    _: ruint8_t,
    _: ruint16_t,
    _: ruint32_t,
    _: ruint64_t,
) {
}
