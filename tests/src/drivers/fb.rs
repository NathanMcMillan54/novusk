use armfb::font::UNICODE_CHARS;

include!("../../../drivers/fb/font.rs");

#[test]
fn armfb_font() {
    let a = 'A';

    for f in 0..UNICODE_CHARS.len() {
        if UNICODE_CHARS[f].0 == a as char {
            for p in 0..UNICODE_CHARS[f].1.len() {
                let pos = UNICODE_CHARS[f].1[p];
                // self.graphics_pixel(self.color, x as u32 + pos.0, y as u32 + pos.1);
                self.draw_square(self.color.0, 3, 3, x as u32 + pos.0 * 3, y as u32 + pos.1 * 3);
            }
        }
    }
}

