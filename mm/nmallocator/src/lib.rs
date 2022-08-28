#![no_std]
#![feature(alloc_error_handler)]

#[macro_use] extern crate cfg_if;

pub mod error;

cfg_if! {
    if #[cfg(feature = "wee")] {
        pub mod wee;
        pub use wee::*;
    }
}

cfg_if! {
    if #[cfg(not(feature = "wee"))] {
        // This could be turned into a proper allocator some day
        pub mod unimplemented;
        pub use unimplemented::*;
    }
}
