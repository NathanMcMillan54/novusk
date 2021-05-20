use esp32::blink::esp32_blink;

pub fn blink() {
    #[cfg(feature = "esp32_board")]
    esp32_blink();
}