use conquer_once::spin::Lazy;
use ps2_mouse::{Mouse, MouseFlags, MouseState};
use spinning_top::Spinlock;
use x86_64::instructions::port::PortReadOnly;
use x86_64::structures::idt::InterruptStackFrame;

pub static MOUSE: Lazy<Spinlock<Mouse>> = Lazy::new(|| Spinlock::new(Mouse::new()));

pub fn ps2_mouse_init() {
    MOUSE.lock().set_on_complete(mouse_finish_init);
    MOUSE.lock().init().unwrap();
}

fn mouse_finish_init(mouse_state: MouseState) {

}

pub fn ps2_mouse_info() -> MouseState {
    return MOUSE.lock().get_state();
}
