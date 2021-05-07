extern "C" {
    fn rpi3_blink();
}

pub unsafe fn blink(i: &str) {
    if i == "rpi3" {
        rpi3_blink();
    }
}

