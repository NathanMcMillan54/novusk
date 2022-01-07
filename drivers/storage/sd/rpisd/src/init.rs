fn rpi_sdcard_init() {

}

fn rpi_sdcard_end() {

}

module_init!(storage_device_init, rpi_sdcard_init);
module_init!(storage_device_end, rpi_sdcard_end);
