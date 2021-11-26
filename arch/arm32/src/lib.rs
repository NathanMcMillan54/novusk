#![no_std]
#![feature(lang_items)]

#[macro_use] extern crate cfg_if;
extern crate nmallocator;

cfg_if! {
    if #[cfg(feature = "cortex_a")] {
        // Cortex A crates
    } else if #[cfg(feature = "cortex_m")] {
        // Cortex M crates
        #[macro_use] extern crate cortex_m_rt;
    }
}

pub mod boot;
pub mod kernel;

#[cfg(feature = "cortex_a")]
pub(crate) mod cortex_a;

#[cfg(feature = "cortex_m")]
pub(crate) mod cortex_m;

#[lang = "eh_personality"]
extern "C" fn eh_personality() { }
