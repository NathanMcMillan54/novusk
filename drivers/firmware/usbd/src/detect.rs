use super::Usb;

impl Usb {

    pub async fn detect_usb(&mut self) -> bool {
        let mut detected_usb: bool = false;

        while true {
            detected_usb = true;

            if detected_usb {
                break;
            }
        }

        return detected_usb;
    }
}
