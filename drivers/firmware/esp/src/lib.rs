#![no_std]

#[export_name = "device_init"]
#[no_mangle]
pub extern "C" fn xtensa_esp_init() -> (Result<(), &'static str>, &'static str) {

    return (Ok(()), "ESP");
}
