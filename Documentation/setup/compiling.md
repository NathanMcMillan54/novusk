# Compiling

Novusk supports UEFI and BIOS booting for x86 and Aarch64, to compile Novusk you need the requirements installed 
(``setup/requirements.md``).

This is the main command:
```commandline
make all ARCH=<cpu architecture> ARCH_FAM=<cpu architecture family> BOOT_METHOD=<boot method> DEVICE=<device name>
```

### Aarch64 compiling
```commandline
make all ARCH=aarch64 ARCH_FAM=aarch64 BOOT_METHOD=uefi DEVICE=default_machine
```

### x86 compiling

For x86_64 UEFI:
```commandline
make all ARCH=x86_64 ARCH_FAM=x86 BOOT_METHOD=uefi DEVICE=default_machine
```

x86_64 BIOS:
```commandline
make all ARCH=x86_64 ARCH_FAM=x86 BOOT_METHOD=bios DEVICE=default_machine
```