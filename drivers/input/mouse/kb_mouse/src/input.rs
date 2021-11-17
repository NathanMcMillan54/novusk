use pc_keyboard::{PcKeyboard, KeyCode, KeyEvent, KeyState};

pub fn kb_mouse_input() -> (isize, isize) {
    let mut keyboard = PcKeyboard::new();

    let key = keyboard.read_key_event();

    if key == KeyEvent::new(KeyCode::Numpad2, KeyState::Down) || key == KeyEvent::new(KeyCode::ArrowDown, KeyState::Down) {
        return (-1, 0);
    } else if key == KeyEvent::new(KeyCode::Numpad4, KeyState::Down) || key == KeyEvent::new(KeyCode::ArrowLeft, KeyState::Down) {
        return (0, -1);
    } else if key == KeyEvent::new(KeyCode::Numpad6, KeyState::Down) || key == KeyEvent::new(KeyCode::ArrowRight, KeyState::Down) {
        return (0, 1);
    } else if key == KeyEvent::new(KeyCode::Numpad8, KeyState::Down) || key == KeyEvent::new(KeyCode::ArrowUp, KeyState::Down) {
        return (1, 0);
    } else { return (0, 0); }
}
