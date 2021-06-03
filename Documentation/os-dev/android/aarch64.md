# Android development on Aarch64

This was a joke because Aarch64 Novusk can't do much. If you want to write an Android distribution, follow these steps.

### Create a new project:

```commandline
cargo new android_distro && cd android_distro
cargo new --lib kernel
```

Add ``kernel/`` to the workspace of ``android_distro/Cargo.toml``

Add Novusk to the kernel dependencies:
```toml
[dependencies.novusk]
git = "https://github/new-kernel"
path = "arch/aarch64/"
features = ["android_os"]
```

This will let you use Android Novsuk, now you can start writing!

Added these functions to kernel/src/lib.rs:
```rust
#[no_mangle]
pub extern "C" fn aarch64_android_kernel_init() -> ! {
    /* Your code here */
    asm::wfe()
}

#[no_mangle]
pub extern "C" fn custom_android_kernel_panic() -> ! {
    /* Panic stuff */
    asm::wfe()
}
```

The main kernel starts in ``aarch64_android_kernel_init`` which is where mostly everything will be. The
``custom_android_kernel_panic`` function is your panic handler, you can do whatever you want with it.
