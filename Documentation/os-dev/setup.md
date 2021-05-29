# Setup OS Development

When you start operating system development with Novusk, you need to decide on two things: what architecture[s] you'll
support, and what do you really want to accomplish.

If you want to support x86 you should probably make an operating system, x86 Novusk will let you use the UEFI which is
like a modern BIOS which is very useful for OS development. If you're going to support aarch64 you have one more option,
aarch64 Novusk does support UEFI but you can also use a true bare metal version of Novusk which is a very low level 
kernel that only runs on certain aarch64 boards. Read ``aarch64/support.md`` for a list of supported boards.

---

### Templates

You could clone them OS templates from the [New Kernel](https://github.com/new-kernel) Github organization.

- [Aarch64 OS example](https://github.com/new-kernel/aarch64_template_example) - Read ``README.md`` for compiling and 
  explanations
- [x86 OS example](https://github.com/new-kernel/x86_template_example) - Read ``README.md`` for compiling and
  explanations
- [xtensa app example](https://github.com/new-kernel/xtensa_template_example) - Read ``README.md`` for explanations
