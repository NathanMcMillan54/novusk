use crate::Blue;
use gpu::GpuGraphics;
use libost::traits::Setup;

impl Setup for Blue {
    fn clear_screen(&mut self) {

    }

    fn draw_logo(&mut self) {

    }

    fn write_loading_message(&mut self, msg: &str) {
        let mut graphics = GpuGraphics::new();

        graphics.graphics_print(200, 240, 15, format_args!("{}", msg));
    }
}
