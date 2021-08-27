pub trait Setup {
    fn init(&mut self, loading_msg: &str) {
        self.clear_screen();
        self.draw_logo();
        self.write_loading_message(loading_msg);
    }

    fn clear_screen(&mut self) {

    }

    fn draw_logo(&mut self) {

    }

    fn write_loading_message(&mut self, msg: &str) {

    }
}
