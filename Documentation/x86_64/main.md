# Novusk - x86_64 Novusk

Last edited: 2022/11/5

---

### Compiling:

```commandline
make novusk ARCH=x86_64 FEATURES=<bootloader, boot_method>
```

Since all x86_64 devices are so similar the build command is very simple. If you're going to link something to Novusk 
like a bootloader, add ``OUTPUT_LIB=true`` to get a static library output. Read [features.md](features.md) for a list of
build features and their usages. 

---

### Capabilities

The x86_64 kernel is the most developed architecture Novusk supports. The x86_64 kernel can boot on BIOS and UEFI 
devices with [Bootloader](https://crates.io/crates/bootloader) or [GRUB](https://gnu.org/grub), read 
[bootloaders.md](link) for more information on booting x86_64 Novusk. The kernel is also able to allocate
[memory](link to memory documentation), handle [IRQs](link to IRQ documentation), handle 
[PC keyboard input](link to keyboard documentation) (a builtin keyboard), and it has some 
[graphics](link to VGA documentation) capabilities.

The x86_64 kernel can't be used on x86_32 (ix86) devices, some parts of the kernel are only meant for 64 bit CPUs so 
Novusk should panic while booting if it detects an un[supported CPU](cpus.md).
