use core::result::Result;

pub fn fmt_test() -> i32 {
    let number = 1;
    let letter = "a";
    format_args!("{}{}", number, letter);
    return 1;
}
