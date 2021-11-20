pub mod common;

pub mod esp32;

use device::Device;
use crate::get_board;

#[export_name = "device_init"]
#[no_mangle]
pub extern "C" fn xtesna_esp_init() -> (Result<(), &'static str>, &'static str) {
    let mut esp_board = get_board();

    esp_board.init();

    return (Ok(()), esp_board.name());
}
