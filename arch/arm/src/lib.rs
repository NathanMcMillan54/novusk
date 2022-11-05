#![no_std]
#![feature(
    alloc_error_handler,
    lang_items,
    panic_info_message,
    trait_upcasting
)]

#[macro_use] extern crate alloc;
#[macro_use] extern crate cfg_if;
#[macro_use] extern crate kinfo;
#[cfg(feature = "nmallocator")]
pub(crate) extern crate nmallocator;
#[macro_use] extern crate novuskinc;
#[macro_use] extern crate printk;

cfg_if! {
    if #[cfg(feature = "cortex_a")] {
        // Cortex A crates
    } else if #[cfg(feature = "cortex_m")] {
        // Cortex M crates
        #[macro_use] extern crate cortex_m_rt;
        #[macro_use] extern crate cortex_m_semihosting;
    }
}

cfg_if! {
    if #[cfg(feature = "rpi2")] {
        pub(crate) extern crate rpi;
        pub(crate) extern crate rpi2kernel;
    }
}

#[cfg(feature = "stellaris_6965")]
pub(crate) extern crate stellarisd;

#[cfg(any(feature = "stm32f407"))]
pub(crate) extern crate stmd;

pub mod boot;
pub mod include;
pub mod kernel;
pub mod liba32;
pub mod mm;
pub mod net;

mod dif;

#[lang = "eh_personality"]
extern "C" fn eh_personality() { }
