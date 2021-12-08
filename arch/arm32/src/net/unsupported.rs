fn unsupported_ethernet_init() {
    printk!("Ethernet is not supported\n");
}

fn unsupported_wireless_init() {
    printk!("Wireless is not supported");
}

define_ethernet_init!(unsupported_ethernet_init);
define_wireless_init!(unsupported_wireless_init);
