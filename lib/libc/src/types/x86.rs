pub type c_char = i8;

#[cfg(target_arch = "x86_64")]
pub type c_long = i64;

#[cfg(target_arch = "x86_64")]
pub type c_ulong = u64;

#[cfg(target_arch = "x86")]
pub type c_long = i32;

#[cfg(target_arch = "x86")]
pub type c_ulong = u32;
