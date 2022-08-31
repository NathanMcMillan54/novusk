#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused)]

use core::borrow::Borrow;
use core::ptr;
use _include::{
    c_char, c_double, c_float, c_int, c_long, c_longlong, c_uchar, c_uint, c_ulong, c_ulonglong,
    c_void, size_t,
};

/// cbindgen:derive-eq=
/// cbindgen:derive-neq
#[no_mangle]
#[repr(C)]
pub struct div_t {
    quot: c_int,
    rem: c_int,
}

/// cbindgen:derive-eq=
/// cbindgen:derive-neq
#[no_mangle]
#[repr(C)]
pub struct ldiv_t {
    quot: c_long,
    rem: c_long,
}

/// cbindgen:derive-eq=
/// cbindgen:derive-neq
#[no_mangle]
#[repr(C)]
pub struct lldiv_t {
    quot: c_longlong,
    rem: c_longlong,
}

pub const RAND_MAX: c_int = 2147483647;

pub const EXIT_FAILURE: c_int = 1;
pub const EXIT_SUCCESS: c_int = 0;

/// Maximum length of a multibyte character in the current locale.
pub const MB_CUR_MAX: c_int = 3;

/// Convert a string to a floating-point number.
#[no_mangle]
pub extern "C" fn atof(s: *const c_char) -> c_double {
    unimplemented!()
}

/// Convert a string to an integer.
#[no_mangle]
pub extern "C" fn atoi(s: *const c_char) -> c_int {
    unimplemented!()
}

/// Convert a string to a long integer.
#[no_mangle]
pub extern "C" fn atol(s: *const c_char) -> c_long {
    unimplemented!()
}

/// Convert a string to a long long integer.
#[no_mangle]
pub extern "C" fn atoll(s: *const c_char) -> c_longlong {
    unimplemented!()
}

/// Convert a string to a floating-point number.
#[no_mangle]
pub extern "C" fn strtod(n_ptr: *const c_char, end_pointer: *const *const c_char) -> c_double {
    unimplemented!()
}

/// Convert a string to a float.
#[no_mangle]
pub extern "C" fn strtof(s: *const c_char, endp: *mut *mut c_char) -> c_float {
    unimplemented!()
}

/// Convert a string to a long double.
#[no_mangle]
pub extern "C" fn strtold(s: *const c_char, endp: *mut *mut c_char)
/* c_longdouble needs implemented */
{
    unimplemented!("c_longdouble needs implemented")
}

/// Convert a string to a long integer.
#[no_mangle]
pub extern "C" fn strtol(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_long {
    unimplemented!()
}

/// Convert a string to a quadword (long long) integer.
#[no_mangle]
pub extern "C" fn strtoq(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_longlong {
    unimplemented!()
}

/// Convert a string to an unsigned long integer.
#[no_mangle]
pub extern "C" fn strtoul(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_ulong {
    unimplemented!()
}

/// Convert a string to an unsigned quadword (long long) integer.
#[no_mangle]
pub extern "C" fn strtouq(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_ulonglong {
    unimplemented!()
}

/// Convert a string to an quadword (long long) integer.
#[no_mangle]
pub extern "C" fn strtoll(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_longlong {
    unimplemented!()
}

/// Convert a string to an unsigned quadword (long long) integer.
#[no_mangle]
pub extern "C" fn strtoull(s: *const c_char, endp: *mut *mut c_char, base: c_int) -> c_ulonglong {
    unimplemented!()
}

/// Convert a floating-point number to a string.
#[no_mangle]
pub extern "C" fn strfromd(
    s: *mut c_char,
    buf_size: size_t,
    format: *const c_char,
    d: c_double,
) -> *mut c_char {
    unimplemented!()
}

/// Convert a floating-point number to a string.
#[no_mangle]
pub extern "C" fn strfromf(
    s: *mut c_char,
    buf_size: c_int,
    format: *const c_char,
    f: c_float,
) -> *mut c_char {
    unimplemented!()
}

/// Convert a floating-point number to a string.
#[no_mangle]
pub extern "C" fn strfroml(
    s: *mut c_char,
    buf_size: c_int,
    format: *const c_char,
    l: c_long,
) -> *mut c_char {
    unimplemented!()
}

#[rustfmt::skip]
#[no_mangle]
pub static conv_table_l64a: [c_uchar; 64] = [
    b'.', b'/', b'0', b'1', b'2', b'3', b'4', b'5',
    b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D',
    b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L',
    b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T',
    b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b',
    b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j',
    b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r',
    b's', b't', b'u', b'v', b'w', b'x', b'y', b'z',
];

const XX: c_uchar = 0xb0;

#[rustfmt::skip]
#[no_mangle]
pub static conv_table_a64l: [c_uchar; 0x4d] = [
    /* 0x2e */                                                           0,  1,
    /* 0x30 */   2,  3,  4,  5,  6,  7,  8,  9, 10, 11, XX, XX, XX, XX, XX, XX,
    /* 0x40 */  XX, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    /* 0x50 */  27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, XX, XX, XX, XX, XX,
    /* 0x60 */  XX, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52,
    /* 0x70 */  53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63
];

/// Convert n to base 64 using the digits "./0-9A-Za-z", least-significant digit first. Returns a
/// pointer to static storage overwritten by the next call.
#[no_mangle]
pub extern "C" fn l64a(n: c_long) -> *const c_uchar {
    let mut n = n as c_ulong;
    let mut result: [c_uchar; 6] = [0; 6];

    // The standard for l64a says that it only uses 32 bits.
    n &= 0xffffffff;
    if n == 0 {
        return conv_table_l64a[0] as *const c_uchar;
    }

    let mut cnt: c_int = 0;
    while cnt > 0 {
        result[cnt as usize] = conv_table_l64a[(n & 0x3f) as usize];
        n >>= 6;
        cnt += 1;
    }

    result.as_ptr()
}

/// Read a number from a string s in base 64 as above.
#[no_mangle]
pub extern "C" fn a64l(s: *const c_uchar) -> c_long {
    let ptr: *const c_uchar = s;
    let mut result: c_ulong = 0;
    let end: *const c_uchar = unsafe { ptr.add(6) }; // TODO: Figure out why this is 6
    let mut shift: c_int = 0;

    loop {
        let index: c_uint = (unsafe { *ptr } - 0x2e) as c_uint;

        if index as c_uint >= 0x4d {
            break;
        }

        let value = conv_table_a64l[index as usize] as c_uint;
        if value == XX as u32 {
            break;
        }
        unsafe {
            ptr.add(1);
        }
        result |= (value as u64) << shift;
        shift += 6;

        if ptr >= end {
            break;
        }
    }

    result as c_long
}

/// Return a random long integer in the range 0 to [`RAND_MAX`] inclusive.
#[no_mangle]
pub extern "C" fn random(_: c_void) -> c_long {
    unimplemented!()
}

/// Seed the random number generator with the given seed.
#[no_mangle]
pub extern "C" fn srandom(seed: c_uint) {
    unimplemented!()
}

/// Initialize the random number generator to use state buffer STATEBUF, of length STATELEN, and
/// seed it with SEED.  Optimal lengths are 8, 16, 32, 64, 128 and 256, the bigger the better;
/// values less than 8 will cause an error and values greater than 256 will be rounded down.
#[no_mangle]
pub extern "C" fn initstate(seed: c_uint, statebuf: *mut c_char, statelen: c_int) -> *mut c_char {
    unimplemented!()
}

/// Switch the random number generator to state buffer STATEBUF, which should have been initialized
/// by [`initstate`].
#[no_mangle]
pub extern "C" fn setstate(statebuf: *mut c_char) -> *mut c_char {
    unimplemented!()
}

/// Reentrant versions of the `random' family of functions. These functions all use the following
/// data structure to contain state, rather than global state variables.
#[no_mangle]
#[repr(C)]
pub struct random_data {
    pub front_ptr: *mut i32,
    pub back_ptr: *mut i32,
    pub state: *mut i32,
    pub rand_type: c_int,
    pub rand_deg: c_int,
    pub rand_sep: c_int,
    pub end_ptr: *mut i32,
}

/// Reentrant version of [`random`].
#[no_mangle]
pub extern "C" fn random_r(data: *mut random_data, result: *mut c_int) -> c_int {
    unimplemented!()
}

/// Reentrant version of [`srandom`].
#[no_mangle]
pub extern "C" fn srandom_r(seed: c_uint, data: *mut random_data) -> c_int {
    unimplemented!()
}

/// Reentrant version of [`initstate`].
#[no_mangle]
pub extern "C" fn initstate_r(
    seed: c_uint,
    statebuf: *mut c_char,
    statelen: c_int,
    data: *mut random_data,
) -> c_int {
    unimplemented!()
}

/// Reentrant version of [`setstate`].
#[no_mangle]
pub extern "C" fn setstate_r(statebuf: *mut c_char, data: *mut random_data) -> c_int {
    unimplemented!()
}

/// cbindgen:ignore
mod random {}

/// Return a random long integer in the range 0 to [`RAND_MAX`] inclusive.
#[no_mangle]
pub extern "C" fn rand(_: c_void) -> c_int {
    unimplemented!()
}

/// Seed the random number generator with the given seed.
#[no_mangle]
pub extern "C" fn srand(seed: c_uint) {
    unimplemented!()
}

/// Reentrant version of [`rand`].
#[no_mangle]
pub extern "C" fn rand_r(seed: *const c_uint) -> c_int {
    unimplemented!()
}

/// Allocate size bytes of memory.
#[no_mangle]
pub extern "C" fn malloc(size: size_t) -> *mut c_void {
    unimplemented!()
}

/// Allocate nmemb elements of size bytes each, all initialized to 0.
#[no_mangle]
pub extern "C" fn calloc(nmemb: size_t, size: size_t) -> *mut c_void {
    unimplemented!()
}

/// Re-allocate the previously allocated memory block pointed to by ptr, making the new memory block
/// size bytes long.
#[no_mangle]
pub extern "C" fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void {
    unimplemented!()
}

/// Re-allocate the previously allocated memory block pointed to by ptr, making the new memory block
/// large enough for nmemb elements of size bytes each.
#[no_mangle]
pub extern "C" fn reallocarray(ptr: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void {
    unimplemented!()
}

/// Free a memory block previously allocated by [`malloc`], [`calloc`], or [`realloc`].
#[no_mangle]
pub extern "C" fn free(ptr: *mut c_void) {
    unimplemented!()
}

/// Allocate size bytes on a page boundary. The storage cannot be freed with [`free`].
#[no_mangle]
pub extern "C" fn valloc(size: size_t) -> *mut c_void {
    unimplemented!()
}

/// ISO C variant of aligned allocation
#[no_mangle]
pub extern "C" fn aligned_alloc(alignment: size_t, size: size_t) -> *mut c_void {
    unimplemented!()
}

/// Abort execution and generate a core dump.
#[no_mangle]
pub extern "C" fn abort() -> ! {
    unimplemented!()
}

// TODO: atexit function
// TODO: at_quick_exit function
// TODO: on_exit function

/// Call all functions registered with [`atexit`] and [`on_exit`] in the reverse of the order in
/// which they were registered, perform any necessary cleanup, and terminate program execution with
/// status.
#[no_mangle]
pub extern "C" fn exit(status: c_int) -> ! {
    unimplemented!()
}

/// Call all functions registered with [`at_quick_exit`] in the reverse of the order in which they
/// were register, and terminate program execution with status.
#[no_mangle]
pub extern "C" fn quick_exit(status: c_int) -> ! {
    unimplemented!()
}

/// Terminate program execution immediately, without calling any of the registered exit functions.
#[no_mangle]
pub extern "C" fn _Exit(status: c_int) -> ! {
    unimplemented!()
}

/// Return the value of the environment variable name, or NULL if the variable is not defined.
///
/// # Safety
/// `from_ptr` is unsafe because it is undefined behavior to pass a null pointer.
#[no_mangle]
pub unsafe extern "C" fn getenv(name: *const c_char) -> *mut c_char {
    unimplemented!()
}

/// The [`clearenv`] function was planned to be added to POSIX.1 but probably never made it into
/// the standard. Nevertheless, the POSIX.9 standard (POSIX bindings for Fortran 77) requires it.
#[no_mangle]
pub extern "C" fn clearenv(_: c_void) {
    unimplemented!() // This will never be implemented.
}

/// Return a [`malloc`]'d string containing the canonical name of the supplied path.
#[no_mangle]
pub extern "C" fn canonicalize_file_name(name: *const c_char) -> *mut c_char {
    unimplemented!()
}

// TODO: realpath function

/// Return the absolute value of int x.
#[no_mangle]
pub extern "C" fn abs(x: c_int) -> c_int {
    if x < 0 {
        -x
    } else {
        x
    }
}

/// Return the absolute value of long int x.
#[no_mangle]
pub extern "C" fn labs(x: c_long) -> c_long {
    if x < 0 {
        -x
    } else {
        x
    }
}

/// Return the absolute value of long long int x.
#[no_mangle]
pub extern "C" fn llabs(x: c_longlong) -> c_longlong {
    if x < 0 {
        -x
    } else {
        x
    }
}

/// Return the [`div_t`] representation of integer division of numer/denom.
#[no_mangle]
pub extern "C" fn div(numer: c_int, denom: c_int) -> div_t {
    div_t {
        quot: numer / denom,
        rem: numer % denom,
    }
}

/// Return the [`ldiv_t`] representation of long integer division of numer/denom.
#[no_mangle]
pub extern "C" fn ldiv(numer: c_long, denom: c_long) -> ldiv_t {
    ldiv_t {
        quot: numer / denom,
        rem: numer % denom,
    }
}

/// Return the [`lldiv_t`] representation of long long integer division of numer/denom.
#[no_mangle]
pub extern "C" fn lldiv(numer: c_longlong, denom: c_longlong) -> lldiv_t {
    lldiv_t {
        quot: numer / denom,
        rem: numer % denom,
    }
}

/// Test if the Rust-ffi interface is working.
#[no_mangle]
pub extern "C" fn rust_ffi_test() -> c_int {
    1
}

/// Use the types required for CBindgen
#[no_mangle]
pub extern "C" fn _use_div_types() {
    let d = div_t {
        quot: 0,
        rem: 0,
    };
    let ld = ldiv_t {
        quot: 0,
        rem: 0,
    };
    let lld = lldiv_t {
        quot: 0,
        rem: 0,
    };

    d.borrow();
    ld.borrow();
    lld.borrow();
}
