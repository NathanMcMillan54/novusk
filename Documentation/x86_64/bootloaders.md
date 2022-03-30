# Novusk - x86_64 bootloader documentation

---

Novusk v3 currently supports [Bootloader](https://crates.io/crates/bootloader) and 
[Grub](https://www.gnu.org/software/grub/), this page will explain how to use Novusk with both

## Bootloader:

Bootloader Rs is a really simple because it links the bootloader to the kernel/Os very easily. If you are using 
Bootloader Rs to boot your Novusk project, you only need to run the following command with the ``bootloader_rs`` 
feature in your ``Cargo.toml``.

For Bootloader Rs v9.x.x:
```command line
cargo bootimage --target x86_64-novusk.json
```

You can run this file on an FAT formatted drive. Bootloader Rs doesn't support multiboot, so the OS executable won't 
stay on the device you're running it on, this is helpful for quick testing. You can use 
[chaing loading](https://en.wikipedia.org/wiki/Chain_loading) with Grub for multiboot.

## Grub:

<!-- For using Grub you should use the ``grub`` option for the 
[Novusk development tools](https://github.com/new-kernel/novusk-dev-tools) to get the files to make a Grub booted OS. -->

To use Grub for booting Novusk, use the ``grub`` feature for the Novusk dependency and make your project a static 
library (this makes linking easier).

```toml
[lib]
name = "your_project"
eddition = "2021"
crate_type = ["staticlib"]


[dependencies.novusk]
git = "https://github.com/new-kernel/novusk"
tag = "v3" # Or  current version
features = ["uefi_boot", "grub"]

```

#### Compiling Grub booted OS

You should put the boot files in an architecture specific directory or just a directory called "boot", it doesn't matter. Your multi boot file needs to call an external function called ``grub_start_novusk`` with the value of the "bootinfo" address as an argument.

Compile project:
```command line
cargo build --target x86_64-novusk.json
ld multiboot target/x86_64-novusk/debug/libyour_project.a -o your_project_os
```

Make a file named ``grub.cfg``, this will link the Grub bootloader with your OS. Read Grub's documentation on it.
