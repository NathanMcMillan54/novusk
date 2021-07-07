#![no_std]
#![feature(asm)]

#[macro_use] extern crate cortex_m_rt;

pub mod boot;

#[cfg(feature = "nrf")]
pub mod nrf;