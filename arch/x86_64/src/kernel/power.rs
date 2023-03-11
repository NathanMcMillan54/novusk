use x86_64::instructions::port::Port;

#[no_mangle]
pub unsafe extern "C" fn sys_reboot() {
    reboot();
}

#[no_mangle]
pub unsafe extern "C" fn sys_shutdown() {
    shutdown();
}

pub unsafe fn reboot() -> ! {
    Port::new(0x64).write(0xfeu8);

    //printk!("Didn't reboot properly, waiting a few cycles before panicking...\n");
    for c in 0..100000000 { }

    panic!("Couldn't reboot from kernel");
}

pub unsafe fn shutdown() -> ! {
    Port::new(0xb004).write(0x2000 as u16);
    Port::new(0x604).write(0x2000 as u16);
    Port::new(0x404).write(0x3400 as u16);
    Port::new(0x64).write(0xf3u8);

    // printk!("Didn't shutdown properly, waiting a few cycles before panicking...\n");

    for c in 0..100000000 { }

    panic!("Couldn't shutdown from kernel");
}
