# Setup OS Development

When you start operating system development with Novusk, you need to decide on two things: what architecture[s] you'll
support, and what do you really want to accomplish.

If you want to support x86 you should probably make an operating system, x86 Novusk will let you use the UEFI which is
like a modern BIOS which is very useful for OS development. If you're going to support aarch64 you have one more option,
aarch64 Novusk does support UEFI but you can also use a true bare metal version of Novusk which is a very low level 
kernel that only runs on certain aarch64 boards. Read ``aarch64/support.md`` for a list of supported boards.

### Project setup

```commandline
cargo new --bin novusk_project --edition 2018 && cd novusk_project
cargo new --lib kernel --edition 2018
touch Makefile
```

Now you should have a binary with a library named ``kernel`` in it's workspace.

After your project directory should look like this:
```
novusk_project
|----.gitignore
|----Cargo.toml
|----Makefile
|----kernel/
|    |----.gitignore
|    |----Cargo.toml
|    |----src/
|         |---lib.rs
|
|----src/
     |----main.rs
```

Now edit your ``Makefile`` so it looks like this:
```makefile
ARCH =? x86
TARGET = $(ARCH)_64-unknown-uefi
VERSION = 2

all: 
    @ cargo build --target $(TARGET)
    @ cd kernel/novusk/ && make build_tools && mv tools/disk_img/target/debug/disk_img ../../disk_img


install:
    @ cd kernel/ && git clone https://github.com/new-kernel/novusk.git && cd kernel/novusk
    @ git checkout --tag $(VERSION)
    @ cp -r arch/$(ARCH)/targets/* ../../

clean:
    @ cargo clean
    @ rm -rf kernel/novusk/
```

The value of ``ARCH`` is the architecture that you will be using but you can change it. The value of ``TARGET`` is the
name of the cross compiler you'll be using, in this example it's value is ``ARCH``_64-unknown-uefi which is 
x86_64-unknown-uefi; that can also be changed. ``VERSION`` is the version of Novusk that you'll be using.

Now edit the main ``Cargo.toml`` so it looks like this:
```toml
[package]
name = "novusk_project"
version = "0.1.0"
edition = "2018"
authors = ["Your Name <your@email.com>"]

[workspace]
members = [
    "kernel/",
    # This can be changed
    "kernel/novusk/arch/x86/",
]

[dependencies]
kernel = { path = "kernel/" }
```

This includes the ``kernel/`` directory and an architecture specific Novusk, don't add Novusk to the dependencies.

Now edit the ``src/main.rs`` and ``kernel/src/lib.rs`` to look like this:
```rust
// src/main.rs
#![no_std]
#![no_main]

extern crate kernel;
```

```rust
// kernel/src/lib.rs
#![no_std]

#[macro_use]
extern crate x86_novusk;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    loop {  }
}
```

Novusk will call a function named ``kernel_main``, this will be the start to you OS or whatever you're making. 

Now back to the ``Makefile``, the function ``all`` will compile your project with the target you give it. The 
``install`` function installs Novusk and sets the version to ``VERSION``. The clean function just removes the 
``target/`` directory,``kernel/novusk/``, and ``disk_img``.

### Compiling

To compile your Novusk project, run this command:
```commandline
make all ARCH=x86 TARGET=x86_64-unknown-uefi VERSION=2
./disk_img target/x86_64-unknown-uefi/debug/novusk_project.fat os_image
```

### Running
```commandline
qemu-system-x86_64 os_image -bios kernel/novusk/x86/OVMF-pure-efi.fd
```
