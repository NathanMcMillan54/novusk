#[cfg(feature = "cortex_m")]
pub use crate::cortex_m::ints::*;

#[cfg(feature = "cortex_a")]
pub use crate::cortex_a::ints::int_init;
