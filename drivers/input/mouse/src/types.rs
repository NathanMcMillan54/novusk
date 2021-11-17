use core::default::Default;

pub type LeftButton = bool;
pub type RightButton = bool;

#[derive(Copy, Clone)]
pub struct MouseCursor {
    pub pos: (usize, usize),
}

impl MouseCursor {
    pub fn new() -> Self {
        return MouseCursor { pos: (20, 20) };
    }

    pub fn update(&mut self, x: usize, y: usize) {
        self.pos = (x, y);
    }

    pub fn current_pos(&mut self) -> (usize, usize) {
        return self.pos;
    }
}

impl Default for MouseCursor {
    fn default() -> Self {
        return MouseCursor { pos: (20, 20) };
    }
}
