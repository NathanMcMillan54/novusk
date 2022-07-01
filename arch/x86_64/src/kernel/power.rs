use printk::printk;
use x86_64::instructions::port::Port;

#[no_mangle]
pub unsafe extern "C" fn reboot() -> ! {
    Port::new(0x64).write(0xfeu8);

    printk!("Failed to reboot, attempting to shutdown...\n");

    shutdown();
}

#[no_mangle]
pub unsafe extern "C" fn shutdown() -> ! {
    Port::new(0xb004).write(0x2000 as u16);
    Port::new(0x604).write(0x2000 as u16);
    Port::new(0x404).write(0x3400 as u16);
    Port::new(0x64).write(0xf3u8);

    printk!("Shutdown failed\n");

    loop {  }
}
