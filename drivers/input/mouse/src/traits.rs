use crate::types::*;

pub trait MouseDevice {
    fn pos(&mut self) -> (i8, i8) {
        // (x, y)
        (0, 0)
    }

    fn btn_up(&mut self) -> (LeftButton, RightButton) {
        // (L, R)
        (false, false)
    }

    fn btn_down(&mut self) -> (LeftButton, RightButton) {
        // (L, R)
        (false, false)
    }

    fn mouse_info(&mut self) -> ((i8, i8), (LeftButton, RightButton), (LeftButton, RightButton)) {
        let pos = self.pos();
        let up = self.btn_up();
        let down = self.btn_down();

        return (pos, up, down);
    }
}
