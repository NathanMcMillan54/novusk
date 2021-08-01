// memory.h
use crate::{c_int, c_macro_int};

#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: c_macro_int) -> c_int {
    let mut i = 0;
    while i < n {
        let a = *s1.offset(i as isize);
        let b = *s2.offset(i as isize);
        if a != b {
            return a as i32 - b as i32
        }
        i += 1;
    }

    return 0;
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8, src: *const u8, n: c_macro_int) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }

    return dest;
}

#[no_mangle]
pub unsafe extern "C" fn memmove(dest: *mut u8, src: *const u8, n: c_macro_int) -> *mut u8 {
    if src < dest as *const u8 {
        let mut i = n;
        while i != 0 {
            i -= 1;
            *dest.offset(i as isize) = *src.offset(i as isize);
        }
    } else {
        let mut i = 0;
        while i < n {
            *dest.offset(i as isize) = *src.offset(i as isize);
            i += 1;
        }
    }

    return dest;
}

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: c_int, n: c_macro_int) -> *mut u8 {
    let mut i = 0;

    while i < n {
        *s.offset(i as isize) = c as u8;
        i += 1;
    }

    return s;
}
