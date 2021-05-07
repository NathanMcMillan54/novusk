use led::blink;

pub unsafe fn blink() {
    #[cfg(feature = "board_rpi3")]
    blink::blink("rpi3");
}