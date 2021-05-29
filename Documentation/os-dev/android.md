# Android development

This was a joke because Aarch64 Novusk can't do much. If you want to write an Android distribution, follow these steps.

### Create a new project:

Clone the Aarch64 OS template:
```commandline
git clone https://github.com/new-kernel/aarch64_template_example.git
```

Rewrite your ``Cargo.toml`` for Android:
```toml
[dependencies.aarch64_novusk]
path = "kernel/novusk/arch/aarch64/"
features = ["android_os"]
```

This will let you use Android Novusk. Because Aarch64 Novusk can't do much and this is a joke, you need to write
everything yourself.

Add the required functions to ``kernel/src/lib.rs``:
```rust
#[no_mangle]
pub unsafe extern "C" fn aarch64_android_kernel_init() -> ! {
    // Setup
    loop { asm!("wfe"); }
}

#[no_mangle]
pub unsafe extern "C" fn custom_android_panic(_info: &PanicInfo) {
    loop { asm!("wfe"); }
}
```

The main kernel starts in ``aarch64_android_kernel_init`` which is where mostly everything will be. The 
``custom_android_kernel_panic`` function is your panic handler, you can do whatever you want with it.
