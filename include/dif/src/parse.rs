use alloc::vec::Vec;
use crate::Dif;

impl Dif {
    pub fn parse(&mut self, file: &str) -> Dif {
        let mut trimmed_file = file.replace("{", "").replace("}", "").replace("\"", "").replace(":", "");
        let mut file_fields: Vec<&str> = trimmed_file.split("\n").collect();

        let mut gen_dif = Dif {
            device_name: None,
            peripheral_addr: None,
            gpio0_addr: None,
            gpio1_addr: None,
            gpio2_addr: None,
            gpio3_addr: None,
            gpio4_addr: None,
            serial_addr: None,
            uart_addr: None,
            fb_addr: None,
            mb_addr: None,
            debug: None
        };

        for f in 0..file_fields.len() {
            if file_fields[f] == "name" {
                gen_dif.device_name = Some(file_fields[f].replace("name ", ""));
            } else if file_fields[f] == "peripheral_address " {
                gen_dif.peripheral_addr = Some(file_fields[f].replace("peripheral_address", ""));
            } else if file_fields[f] == "gpio0_address " {

            } else if file_fields[f] == "gpio1_address" {

            } else if file_fields[f] == "gpio2_address" {

            } else if file_fields[f] == "gpio3_address" {

            } else if file_fields[f] == "gpio4_address" {

            } else if file_fields[f] == "serial_address " {

            } else if file_fields[f] == "uart_address " {

            } else if file_fields[f] == "fb_address " {

            } else if file_fields[f] == "mb_address " {

            } else if file_fields[f] == "debug " { }
        }

        return gen_dif;
    }
}
