use core::fmt::{Arguments, Write};
use super::{DriverNames, DRIVER};

#[cfg(target_arch = "x86_64")]
fn vga_print(x: usize, y: usize, color: usize, args: Arguments) {
    use vgag::{Color16, display::VgaDisplay};

    let vga_color = super::color::convert_to_vga_color(color);
    let mut vga = VgaDisplay::new(x, y, vga_color);

    vga.write_fmt(args);
}

#[no_mangle]
pub extern "C" fn graphics_print(x: usize, y: usize, color: usize, args: Arguments) {
    unsafe {
        if DRIVER == DriverNames::Vgag {
            #[cfg(target_arch = "x86_64")]
            vga_print(x, y, color, args);
        } else {
            return;
        }
    }
}
