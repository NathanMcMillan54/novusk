/// Because CortexM devices are usually simpler, the CortexM kernel is separate from the
/// CortexA kernel because it doesn't have to do as much. This module contains code intended for
/// CortexM CPUs but will eventually call the main kernel like all other architectures and specific
/// CPUs.
#[cfg(feature = "cortex_m_device")]
pub mod cm_kernel;
