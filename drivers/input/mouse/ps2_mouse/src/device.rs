use mouse::MouseDevice;
use crate::Ps2Mouse;
use ps2_mouse::{Mouse, MouseState};
use spin::Lazy;
use spinning_top::Spinlock;
use kinfo::status::set_status;
use x86_64::instructions::port::Port;

static PS2MOUSE: Lazy<Spinlock<Mouse>> = Lazy::new(|| Spinlock::new(Mouse::new()));

unsafe fn init_error(error: &str) {
    set_status("not ok");
    kinfo!("Failed to initialize PS2 Mouse");
    set_status("not ok");
    printk!("    {}", error);
}

impl Ps2Mouse {
    pub fn init(&mut self) {
        let mouse_init = match PS2MOUSE.lock().init() {
            Ok(..) => return,
            Err(E) => unsafe { init_error(E) },
        };
    }

    fn mouse_state(&mut self) -> MouseState {
        unsafe { PS2MOUSE.lock().process_packet(Port::new(0x60).read()); }
        return PS2MOUSE.lock().get_state();
    }
}

impl MouseDevice for Ps2Mouse {
    fn pos(&mut self) -> (i8, i8) {
        return (self.mouse_state().get_x() as i8, self.mouse_state().get_y() as i8);
    }

    fn btn_up(&mut self) -> (bool, bool) {
        return (self.mouse_state().left_button_up(), self.mouse_state().right_button_up());
    }

    fn btn_down(&mut self) -> (bool, bool) {
        return (self.mouse_state().left_button_down(), self.mouse_state().right_button_down());
    }
}
