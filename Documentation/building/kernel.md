# Novusk - Compiling Novusk on its own

---

If you're compiling Novusk to get a binary a static library for just running or linking, this page will explain how to
do so. [Here](link) is a link to build setup which shows how install build requirements and talks about other things 
that are required for building.


## Compiling Novusk binary:

This is the main command for compiling Novusk:

```commandline
cargo build --<release/debug> --features=<kernel/features.md> --target <kernel/targets.md>
```

Novusk might need additional linking depending on the device it was compiled for, the [devices](todo) documentation has
information on the devices Novusk supports and how to compile Novusk for them properly.
