#![no_std]

pub mod input;
pub mod layout;

pub use layout::get_pckeyboard_layout;

pub struct PcKeyboard;

impl PcKeyboard {
    pub fn new() -> Self {
        return PcKeyboard;
    }
}
