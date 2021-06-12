use crate::color::switch_color;
use libefi::st;
use net::{set_net};
use uefi::proto::console::text::Color;

pub unsafe fn box_init() {
    switch_color(Color::LightGreen, Color::Black);
    kinfo!("Green text initialized");

    set_net("ethernet");
}
