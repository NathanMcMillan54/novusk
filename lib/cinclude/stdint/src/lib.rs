#![no_std]
#![feature(lang_items)]

pub use stdint_intn::*;
pub use stdint_uintn::*;

#[cfg(not(feature = "no_lang_items"))]
#[path = "../../lang.rs"]
pub(crate) mod lang;

// Small types
// TODO: Small types

// Fast types
// TODO: Fast types

// Types for void* pointers
// TODO: Types for void* pointers

// Largest integral types
// TODO: Largest integral types

// Limits of integral types
// Minimum signed integer types
pub const INT8_MIN: i8 = -128; // -2^7
pub const INT16_MIN: i16 = -32767 - 1; // TODO: Figure out why this needs -1 // -2^15
pub const INT32_MIN: i32 = -2147483647 - 1; // TODO: Figure out why this needs -1 // -2^31
#[cfg(target_pointer_width = "64")]
pub const INT64_MIN: i64 = -9223372036854775807 - 1; // TODO: Figure out why this needs -1 // -2^63
#[cfg(target_pointer_width = "32")]
pub const INT64_MIN: i32 = -2147483647 - 1; // TODO: Figure out why this needs -1 // -2^31

// Maximum signed integer types
pub const INT8_MAX: i8 = 127; // 2^7 - 1
pub const INT16_MAX: i16 = 32767; // 2^15 - 1
pub const INT32_MAX: i32 = 2147483647; // 2^31 - 1
#[cfg(target_pointer_width = "64")]
pub const INT64_MAX: i64 = 9223372036854775807; // 2^63 - 1
#[cfg(target_pointer_width = "32")]
pub const INT64_MAX: i32 = 2147483647; // 2^31 - 1

// Minimum unsigned integer types
pub const UINT8_MIN: u8 = 0; // 0
pub const UINT16_MIN: u16 = 0; // 0
pub const UINT32_MIN: u32 = 0; // 0
#[cfg(target_pointer_width = "64")]
pub const UINT64_MIN: u64 = 0; // 0
#[cfg(target_pointer_width = "32")]
pub const UINT64_MIN: u32 = 0; // 0

// Maximum unsigned integer types
pub const UINT8_MAX: u8 = 255; // 2^8 - 1
pub const UINT16_MAX: u16 = 65535; // 2^16 - 1
pub const UINT32_MAX: u32 = 4294967295; // 2^32 - 1
#[cfg(target_pointer_width = "64")]
pub const UINT64_MAX: u64 = 18446744073709551615; // 2^64 - 1
#[cfg(target_pointer_width = "32")]
pub const UINT64_MAX: u32 = 4294967295; // 2^32 - 1

// Minimum small integer types
// TODO: Minimum small integer types

// Maximum small integer types
// TODO: Maximum small integer types

// Minimum fast integer types
// TODO: Minimum fast integer types

// Maximum fast integer types
// TODO: Maximum fast integer types

// Values to test for integral types holding void* pointers
// TODO: Values to test for integral types holding void* pointers

// Integral type widths
pub const INT8_WIDTH: usize = 8;
pub const INT16_WIDTH: usize = 16;
pub const INT32_WIDTH: usize = 32;
#[cfg(target_pointer_width = "64")]
pub const INT64_WIDTH: usize = 64;
#[cfg(target_pointer_width = "32")]
pub const INT64_WIDTH: usize = 32;

pub const UINT8_WIDTH: usize = 8;
pub const UINT16_WIDTH: usize = 16;
pub const UINT32_WIDTH: usize = 32;
#[cfg(target_pointer_width = "64")]
pub const UINT64_WIDTH: usize = 64;
#[cfg(target_pointer_width = "32")]
pub const UINT64_WIDTH: usize = 32;

// TODO: All the other widths
