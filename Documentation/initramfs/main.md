# Initramfs

If you don't know what an initramfs is or what it's for, read ``usage.md``

The function that you would use to start your initramfs is called ``initramfs_main``, it doesn't need a return type.

Usage:
```rust
// kernel/src/initramfs/mod.rs
// If you want an initramfs this function needs to return true
#[no_mangle]
pub extern "C" fn initramfs() -> bool { return true; }

#[no_mangle]
pub extern "C" fn initramfs_main() {
    /* Create Ram Fs */
}
```
