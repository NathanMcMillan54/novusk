use ps2_mouse::{Mouse, MouseState};
use spinning_top::Spinlock;
use x86_64::instructions::port::PortReadOnly;

pub static mut MOUSE: Mouse = Mouse::new();

pub unsafe fn init_mouse() {
    MOUSE.init();
    // This was mostly copied from the example so it makes sense that it doesn't work
    MOUSE.set_on_complete(on_complete);
}

fn on_complete(mouse_state: MouseState) {
    unsafe {
        printk!("Mouse state: {:?}", mouse_state);
    }
}
