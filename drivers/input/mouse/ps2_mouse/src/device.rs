use mouse::MouseDevice;
use crate::Ps2Mouse;
use ps2_mouse::{Mouse};
use spin::Lazy;
use spinning_top::Spinlock;
use kinfo::status::set_status;

static PS2MOUSE: Lazy<Spinlock<Mouse>> = Lazy::new(|| Spinlock::new(Mouse::new()));

impl Ps2Mouse {
    pub fn init(&mut self) {
        let mouse_init = match PS2MOUSE.lock().init() {
            Ok(..) => return,
            Err(E) => {
                unsafe { set_status("not ok") }
                kinfo!("Failed to initialize PS2 Mouse");
                printk!("    {}", E);
            },

            _ => printk!("No info")
        };
    }
}

