pub trait MouseDevice {
    fn pos(&mut self) -> (i8, i8) {
        // (x, y)
        (0, 0)
    }

    fn btn_up(&mut self) -> (bool, bool) {
        // (L, R)
        (false, false)
    }

    fn btn_down(&mut self) -> (bool, bool) {
        // (L, R)
        (false, false)
    }
}
