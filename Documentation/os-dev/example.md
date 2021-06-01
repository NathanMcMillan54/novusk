# Example

kernel/Cargo.toml:
```toml
[dependencies]
libnu = "0.1.0"

[dependencies.novusk_syscalls]
version = "0.1.0"
# If you're using UEFI
features = ["novusk_uefi"]

[target.'cfg(target_arch = "x86_64")'.dependencies]
[dependencies.novusk]
git = "https://github.com/new-kernel/novusk/"
path = "arch/x86/"
features = ["default_machine"]
```

This is for x86_64 architecture, you can set arch to whatever you want.

src/main.rs:
```rust
#![no_std]
#![no_main]

extern crate kernel;
```

kernel/src/lib.rs
```rust
#![no_std]

pub mod required;

#[macro_use] extern crate libnu;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    println!("Hello world!");
    // Start your OS
    loop {  }
}
```

kernel/src/required.rs:
```rust
use libnu::types::ApplicationType;

#[no_mangle]
pub extern "C" fn kernel_info() -> bool { return true; }

#[no_mangle]
pub extern "C" fn application_type() -> ApplicationType { return ApplicationType::OperatingSystem; }

#[no_mangle]
pub extern "C" fn main_color() -> &'static str { return "green"; }

#[no_mangle]
pub extern "C" fn initramfs() -> bool { return false; }

#[no_mangle]
pub extern "C" fn initramfs_main() { return; }
```

To compile get the build target and Efi OVMF file for running
```commandline
curl https://github.com/new-kernel/novusk/blob/master/arch/x86/targets/x86_64-novusk.json > x86_64-novusk-os.json
curl https://github.com/new-kernel/novusk/blob/master/arch/x86/OVMF-pure-efi.fd > OVMF-pure-efi.fd
```

If core, compiler builtins, and compiler memory is in your ``.cargo/config.toml`` you can compile by running:
```commandline
cargo build --target x86_64-novusk-os.json
```

Then run with:
```commandline
qemu-system-x86_64 target/x86_64-novusk-os.json -bios OVMF-pure-efi.fd
```
