pub trait RaspberryPi {
    fn uart_io_init(&mut self) {

    }

    fn gpio_init(&mut self) {

    }

    fn led_on(&self) {

    }

    fn led_off(&self) {

    }

    fn mailbox_init(&mut self) {

    }

    fn fb_init(&mut self) {

    }
}