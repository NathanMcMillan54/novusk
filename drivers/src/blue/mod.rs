pub mod screen;
use screen::clear_screen;

#[no_mangle]
pub extern "C" fn blue_init() {
    clear_screen();
}
