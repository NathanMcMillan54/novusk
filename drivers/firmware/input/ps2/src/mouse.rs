use conquer_once::spin::Lazy;
use input::MouseDevice;
use ps2_mouse::{Mouse, MouseFlags, MouseState};
use spinning_top::Spinlock;

static MOUSE: Lazy<Spinlock<Mouse>> = Lazy::new(|| Spinlock::new(Mouse::new()));

pub struct Ps2Mouse;

impl Ps2Mouse {
    pub fn new() -> Self {
        return Ps2Mouse;
    }

    pub fn init(&mut self) -> (bool, &str) {
        match MOUSE.lock().init() {
            Ok(..) => return (true, "PS2 Mouse initialized successfully"),
            Err(E) => (false, E),
        }
    }

    fn mouse_state(&mut self) -> MouseState {
        return MOUSE.lock().get_state();
    }
}

impl MouseDevice for Ps2Mouse {
    fn pos(&mut self) -> (i32, i32) {
        (self.mouse_state().get_x() as i32, self.mouse_state().get_y() as i32)
    }

    fn btn_up(&mut self) -> (bool, bool) {
        (self.mouse_state().left_button_up(), self.mouse_state().right_button_up())
    }

    fn btn_down(&mut self) -> (bool, bool) {
        (self.mouse_state().left_button_down(), self.mouse_state().right_button_down())
    }
}
