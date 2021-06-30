pub mod id;
pub use id::BRAND;

use super::kernel::*;
use crate::drivers::amd::amd_init;
use crate::drivers::intel::intel_init;
