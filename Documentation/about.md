# About Novusk

Novusk is Rust kernel inspired by [Linux](https://github.com/torvalds/linux/). Since this is written in Rust 
it will be slightly faster and more secure because of Rust's safety features. For more information go to the 
[new-kernel readme](https://github.com/new-kernel/new-kernel). 

###  Goal

The goal is to make Novusk supported on most architectures and devices and for it to be usable for any bare metal
project.

---

### Usage

x86 Novusk usage:

x86_64/i686 Novusk should be used for operating system development because it is written with a UEFI library which is
very useful for OS development.

---

aarch64 Novusk usage:

Just like x86 Novusk, aarch64 UEFI Novusk is good for OS development for aarch64 laptops, desktops, or powerful 
microcomputers. Aarch64 Novusk without the UEFI can be used for simple bare metal projects, like making an LED flash on
a microcontroller, or making a very small storage server (only a few files).

xtensa Novusk usage:

There hasn't been much work put into Xtensa Novusk. Xtensa isn't a widely used architecture and Rust doesn't support it.
Right now, on the Esp32, it should set a network driver (that doesn't do anything) and make the on-board led blink, then
start an Xtensa app. In the future it should run network driver tests and start memory management (all on the Esp32).
