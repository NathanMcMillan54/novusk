use libcolor::html_colors::HtmlColors;
use rpi::RpiFb;

pub fn a64_fb_init() {
    let mut fb = RpiFb::fb_init();

    // fb.clear_screen(HtmlColors::White);
    fb.clear_screen(0xffffff);
}
