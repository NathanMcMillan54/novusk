pub trait KeyboardDevice {
    fn read(&mut self) -> char {
        'c'
    }
}

pub trait MouseDevice {
    fn pos(&mut self) -> (i32, i32) {
        // (x, y)
        (0, 0)
    }

    fn btn_up(&mut self) -> (bool, bool) {
        // (l, r)
        (true, true)
    }

    fn btn_down(&mut self) -> (bool, bool) {
        // (l, r)
        (true, true)
    }
}
