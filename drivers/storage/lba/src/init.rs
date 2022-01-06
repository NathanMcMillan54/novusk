fn lba_init() {

}

fn lba_end() {

}

module_init!(storage_device_init, lba_init);
module_end!(storage_device_end, lba_end);
