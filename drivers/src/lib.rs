#![no_std]

#[macro_use]
extern crate kernel;

pub mod blue;
pub mod ethernet;

pub unsafe fn hardware_drivers_init() {
    ethernet::init();
}
