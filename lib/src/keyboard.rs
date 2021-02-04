use pc_keyboard::*;
use crate::user::print::_printnl;

fn arm_input(mut repeat: i32)  {
    let loops = 999;
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet2, HandleControl::MapLettersToUnicode);
    loop {
        if repeat == 1 { break }
        if repeat == loops { break }
        match keyboard.add_byte(0x20) {
            Ok(Some(event)) => {
                _printnl(format_args!("{:?}", event));
            }
            Ok(none) => {
                _printnl(format_args!("No input"));
            }
            Err(e) => {
                _printnl(format_args!("{:?}", e));
            }
        }
        repeat = repeat + 1;
    }
}

fn x86_input(mut repeat: i32) {
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore);
    let loops = 999;
    loop {
        if repeat == 1 { break }
        if repeat == loops { break }
        match keyboard.add_byte(0x20) {
            Ok(Some(event)) => {
                _printnl(format_args!("{:?}", event));
            }
            Ok(none) => {
                _printnl(format_args!("No input"));
            }
            Err(e) => {
                _printnl(format_args!("{:?}", e));
            }
        }
        repeat = repeat + 1;
    }
}

pub fn single_character_input() -> &'static str {
    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    arm_input(0);

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    x86_input(0);
    return "a"
}

pub fn loop_input() -> &'static str {
    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    arm_input(999);

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    x86_input(999);
    return "a"
}
