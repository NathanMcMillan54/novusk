use crate::color::switch_color;
use libefi::st;
use uefi::proto::console::text::Color;

pub unsafe fn box_init() {
    switch_color(Color::LightGreen, Color::Black);
    kinfo!("Green text initialized");
}
