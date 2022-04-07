# Novusk - x86_64 kernel capabilities and information

---

The x86_64 kernel is probably the most developed architecture Novusk supports. The x86_64 kernel can boot on BIOS and 
UEFI devices with [Bootloader](https://crates.io/crates/bootloader) or [GRUB](https://gnu.org/grub), read 
[bootloaders.md](link) for more information on booting x86_64 Novusk. The kernel is also able to allocate 
[memory](link to memory documentation), handle [IRQs](link to IRQ documentation), handle 
[PC keyboard input](link to keyboard documentation) (the keyboard on a laptop), and it has some 
[graphics](link to VGA documentation) capabilities.

Some parts of Novusk might be able to run on x86_32 (iX86) devices, it will definitely boot but it might not finish all 
it's processes since somethings are 64 but specific.
