/// This file is used for booting on CortexM CPUs, it uses the ``entry`` macro from the
/// ``cortex-m-rt`` library as a "_start" function.
#[cfg(feature = "cortex_m_device")]
pub mod cmb;

pub mod mem;
