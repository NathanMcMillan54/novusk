#![no_std]

#[macro_use] extern crate alloc;
#[macro_use] extern crate cfg_if;
extern crate nkernel;
#[macro_use] extern crate novuskinc;
extern crate r0;

cfg_if! {
    if #[cfg(feature = "cortex_m_device")] {
        extern crate cortex_m;
        #[macro_use] extern crate cortex_m_rt;
    }
}

#[cfg(feature = "lm3s6965")]
extern crate ti_lm3s;

pub mod boot;
pub mod kernel;
pub mod mm;
