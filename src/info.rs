#[no_mangle]
pub extern "C" fn boot_method() -> &'static str {
    #[cfg(feature = "bios_boot")]
    return "BIOS";

    #[cfg(feature = "no_boot")]
    return "None";

    #[cfg(feature = "uefi_boot")]
    return "UEFI";
}

#[no_mangle]
pub extern "C" fn device_name() -> &'static str {
    #[cfg(feature = "default_machine")]
    return "Default";

    #[cfg(feature = "stm32f4")]
    return "STM32F4"
}
