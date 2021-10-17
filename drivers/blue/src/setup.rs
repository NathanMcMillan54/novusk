use crate::Blue;
// use gpu::GpuGraphics;
use libost::traits::Setup;
/* use vgag::display::VgaDisplay;
use vgag::Color16; */

impl Setup for Blue {
    fn clear_screen(&mut self) {
        // let mut graphics = GpuGraphics::new();

        for y in 0..480 {
            for x in 0..640 {
                // graphics.pixel(x, y, 15);
            }
        }
    }

    fn draw_logo(&mut self) {

    }

    fn write_loading_message(&mut self, msg: &str) {
        // let mut graphics = GpuGraphics::new();

        // graphics.graphics_print(200, 240, 15, format_args!("{}", msg));
        // graphics.graphics_print(200, 250, 15, format_args!("{}", "Starting Blue Os..."))
    }
}
