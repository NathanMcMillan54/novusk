use crate::Blue;
use gpu::GpuGraphics;
use libost::traits::Setup;

impl Setup for Blue {
    fn clear_screen(&mut self) {
        let mut graphics = GpuGraphics::new();

        for y in 0..480 {
            for x in 0..640 {
                graphics.draw_pixel(x, y, 3);
            }
        }
    }

    fn draw_logo(&mut self) {

    }

    fn write_loading_message(&mut self, msg: &str) {
        let mut graphics = GpuGraphics::new();

        graphics.write_string(275, 200, 15, "Starting Blue...");
    }
}
