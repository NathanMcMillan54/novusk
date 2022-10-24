// This is probably a terrible way of making a font
pub const UNICODE_CHARS: &[(char, &[(u32, u32)])] = &[
    // "A" character
    ('A', &[(0, !0), (1, 0), (!0, 0), (1, 1), (0, 1), (!0, 1), (!0, 2), (1, 2)]),
    // "a" character (sorta)
    ('a', &[(!0, 0), (0, !0), (1, !0), (2, !1), (2, !0), (2, 0), (2, 1), (!0, 1), (0, 1), (0, 2)])
];
