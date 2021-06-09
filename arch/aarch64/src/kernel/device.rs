pub unsafe fn device_init() {
    #[cfg(feature = "default_machine")] {
        use crate::drivers::device::default;
        default::default_init();
    }

    #[cfg(feature = "uefi_rpi3")] {
        use crate::drivers::device::uefi_rpi3;
        uefi_rpi3::uefi_pi3_init();
    }

    #[cfg(feature = "qemu_virt")] {
        use crate::drivers::device::qemu_virt;
        qemu_virt::qemu_virt_init();
    }
}
