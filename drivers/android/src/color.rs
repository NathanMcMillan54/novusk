use libefi::st;
use uefi::proto::console::text::Color;

pub unsafe fn switch_color(text: Color, background: Color) {
    st().as_ref().stdout().set_color(text, background);
}