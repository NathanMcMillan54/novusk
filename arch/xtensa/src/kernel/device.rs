pub fn device() -> &str {
    #[cfg(feature = "esp32")]
    return "ESP32";
}