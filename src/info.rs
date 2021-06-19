#[no_mangle]
pub extern "C" fn boot_method() -> &'static str {
    #[cfg(feature = "uefi_boot")]
    return "UEFI";

    #[cfg(feature = "bios_boot")]
    return "BIOS";
}
