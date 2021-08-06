#![no_std]
#![feature(asm, global_asm)]
#![feature(alloc_error_handler, alloc_layout_extra, panic_info_message)]
#![feature(stmt_expr_attributes)]

#[macro_use] extern crate alloc;
#[cfg(target_arch = "arm")]
#[macro_use] extern crate cortex_m_rt;
#[macro_use] extern crate kinfo;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "arm")] {
        pub mod boot;
        pub mod kernel;
        pub mod mm;
    }
}

pub mod include;

#[cfg(feature = "rpi")]
pub mod rpi;

#[cfg(feature = "nrf")]
pub mod nrf;
