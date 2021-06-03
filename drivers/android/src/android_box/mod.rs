use libefi::st;
use uefi::proto::console::text::Color;

unsafe fn switch_to_green() {
    let stdout = st().as_ref().stdout();
    stdout.set_color(Color::LightGreen, Color::Black).unwrap();
}

pub unsafe fn box_init() {
    switch_to_green();
    kinfo!("Green text initialized");
}
