# Novusk - x86_64 features

Last edited: 2022/11/5

---

x86_64 Novusk features are used mainly for booting information, like bootloader name, and bootloading method.

- ``bios_boot``, kernel loads from BIOS
- ``bootloader_rs``, kernel was booted with [Bootloader](https://crates.io/crates/bootloader)
- ``grub``, enables ``multiboot``, kernel was booted with [Grub](https://www.gnu.org/software/grub/)
- ``multiboot``, kernel was booted with a bootloader that supports multiboot
- ``uefi_boot``, kernel loads from UEFI
