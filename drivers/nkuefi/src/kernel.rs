extern "C" {
    fn x86_main() -> !;
}

pub unsafe fn start_novusk() -> ! {
    #[cfg(target_arch = "x86_64")]
    x86_main();
}
