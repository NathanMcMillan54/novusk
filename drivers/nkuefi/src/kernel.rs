unsafe fn start_x86_novusk() -> ! {
    extern "C" { fn main() -> !; }
    main();
}

pub(crate) unsafe fn start_novusk() -> ! {
    // #[cfg(target_arch = "x86_64")]
    start_x86_novusk();
}
