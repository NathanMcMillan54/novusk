use uefi::proto::console::text::Input;

pub struct Keyboard;

#[no_mangle]
pub unsafe extern "C" fn keyboard_init(stdin: &mut Input) {

}
