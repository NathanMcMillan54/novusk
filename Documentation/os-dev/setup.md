# Setup OS Development

When you start operating system development with Novusk, you need to decide on two things: what architecture[s] you'll
support, and what do you really want to accomplish.

If you want to support x86 you should probably make an operating system, x86 Novusk will let you use the UEFI which is
like a modern BIOS which is very useful for OS development. If you're going to support aarch64 you have one more option,
aarch64 Novusk does support UEFI, but you can also use a true bare metal version of Novusk which is a very low level 
kernel that only runs on certain aarch64 boards. Read ``aarch64/support.md`` for a list of supported boards.

To get started, make a new Cargo project, clone Novusk and add it to the workspace. After that add the required 
functions, read ``samples/os_example/src/required.rs`` for the required functions and their return types.

Another thing you should know, no matter what architecture you are using the main function is ``kernel_main``.

---

### Examples

The ``sample/`` directory has some examples for x86 Novusk (UEFI Aarch64 is similar). The sample directory has an OS
example, and a kernel extension that starts an OS.

Also read ``example.md`` for writing an OS

---
