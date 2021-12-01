#![no_std]
#![feature(alloc_error_handler, lang_items, panic_info_message)]

#[macro_use] extern crate alloc;
#[macro_use] extern crate cfg_if;
extern crate nmallocator;

cfg_if! {
    if #[cfg(feature = "cortex_a")] {
        // Cortex A crates
    } else if #[cfg(feature = "cortex_m")] {
        // Cortex M crates
        #[macro_use] extern crate cortex_m_rt;
        #[macro_use] extern crate cortex_m_semihosting;
    }
}

#[cfg(feature = "rpi2")]
pub(crate) extern crate rpi;

#[cfg(feature = "stellaris_6965")]
pub(crate) extern crate stellarisd;

pub mod boot;
pub mod kernel;
pub mod mm;

#[cfg(feature = "cortex_a")]
pub(crate) mod cortex_a;

#[cfg(feature = "cortex_m")]
pub(crate) mod cortex_m;

#[lang = "eh_personality"]
extern "C" fn eh_personality() { }
