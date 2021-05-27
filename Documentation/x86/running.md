# Running x86 Novusk

### Physical Hardware
Since x86 Novusk only supports UEFI, getting any Novusk x86 target to run on real hardware should be easy. Put the 
``bzImage`` file from ``src/boot/`` on a FAT32 formatted USB, then put it on a computer that support UEFI/EFI.

### Simulating in Qemu
Qemu doesn't support UEFI, so you need the file ``OVMF-pure-efi.fd`` which is in the main x86 directory.

Main command (for x86_64):
```commandline
qemu-system-x86_64 -drive format=raw,file=src/boot/bzImage -bios OVMF-pure-efi.fd
```

Or shorten it:
```commandline
qemu-system-x86_64 src/boot/bzImage -bios OVMF-pure-efi.fd
```

i686 Novusk:
```commandline
qemu-system-i386 -cpu pentium3 -kernel src/boot/bzImage -bios OVMF-pure-efi.fd
```

Qemu doesn't support i686 or pentium4 CPU and barely supports UEFI, so it's hard to tell if Novusk can run on i686
without running it on real hardware which hasn't been done yet.

The file ``OVMF-pure-efi.fd`` is used as a BIOS replacement so Qemu can run a UEFI kernel/OS.