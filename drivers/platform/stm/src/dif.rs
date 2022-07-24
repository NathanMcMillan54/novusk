#[no_mangle]
pub static DIF_FILE: &'static [(&'static str, &'static str); 11] = &[
    ("DifName", "stm32f407.dif"),
    ("DeviceName", "STM32f407"),
    ("PeripheralAddress", "0"),
    ("AllocMemory", "true"),
    ("EnableSerial", "true"),
    ("PrintingMethod", "Serial"),
    ("DeviceKernel", "false"),
    ("StartInit", "true"),
    ("IrqChip", "NVIC"),
    ("EnableDeviceIrqs", "true"),
    ("ShutdownOnPanic", "false")
]
;
