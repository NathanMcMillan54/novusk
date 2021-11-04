#![no_std]
#![no_main]

#[macro_use] extern crate alloc;
#[macro_use] pub extern crate novusk;

use libwin::{graphics::graphics::*, Window};
use libost::desktop::{Desktop, DesktopIcon};

#[no_mangle]
pub unsafe extern "C" fn kernel_main() {
    printk::printk!("\nKernel Main");

    let mut desktop = Desktop::new((640, 480), CYAN, Some(vec![DesktopIcon::new((16, 16), WHITE)]));
    let mut window = Window::new(Some("Test window"), Some((100, 200)), (100, 200), RED);

    desktop.init();
    window.display();

    loop {  }
}

#[no_mangle]
pub extern "C" fn initramfs_main() {

}
