#![no_std]
#![feature(lang_items)]

use libc::{c_int, c_void, size_t};

#[cfg(not(feature = "no_lang_items"))]
#[path = "../../lang.rs"]
pub(crate) mod lang;

/// Copy `n` bytes from `src` to `dest`
#[no_mangle]
pub extern "C" fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *const c_void {
    unimplemented!()
}

/// Copy `n` bytes from `src` to `dest`, guaranteeing correct behavior for overlapping strings.
#[no_mangle]
pub extern "C" fn memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *const c_void {
    unimplemented!()
}

/// Copy no more than `n` from `src` to `dest`, stopping when `c` is found.
/// Returns the position in `dest` one byte passed where `c` was copied, or NULL if `c` was not
/// found in the first `n` bytes of `src`.
#[no_mangle]
pub extern "C" fn memccpy(
    dest: *mut c_void,
    src: *const c_void,
    c: c_int,
    n: size_t,
) -> *const c_void {
    unimplemented!()
}

/// Set `n` bytes of `s` to `c`
#[no_mangle]
pub extern "C" fn memset(s: *mut c_void, c: c_int, n: size_t) -> *const c_void {
    unimplemented!()
}

/// Compare the first `n` bytes of `s1` to `s2`
#[no_mangle]
pub extern "C" fn memcmp(s1: *const c_void, s2: *const c_void, n: c_int) -> c_int {
    unimplemented!()
}
