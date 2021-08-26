use core::str::from_utf8_unchecked;

pub fn putc(c: u8) {
    let s = vec![c];

    unsafe { puts(from_utf8_unchecked(s.as_slice())); }
}

pub fn puts(s: &str) {
    extern "C" {
        fn writes(s: &str);
    }

    unsafe { writes(s); }
}
