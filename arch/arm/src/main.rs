#![no_std]
#![no_main]

#[cfg(feature = "allocator")]
#[macro_use] extern crate alloc;

#[macro_use] extern crate cfg_if;
extern crate kernel as nkernel;
extern crate setup;

cfg_if! {
    if #[cfg(feature = "cortex_m")] {
        #[macro_use] extern crate cortex_m;
        #[macro_use] extern crate cortex_m_rt;
    }
}

#[cfg(feature = "stellaris_6965")]
pub(crate) extern crate stellarisd;

#[path = "dif.rs"]
pub(crate) mod dif;

pub mod boot;
pub mod kernel;
pub mod liba32;
pub mod mm;
