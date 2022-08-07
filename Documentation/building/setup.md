# Novusk - Setup building

---

## Requirements:

- ``rustc`` - ~1.61
- ``rustup`` - ~1.25
- ``nightly`` - ~1.64

## Install:

Update Rust and install Nightly:
```commandline
rustup update
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly
```

Install any cross-compilers that might be needed:
```commandline
sudo apt-get update
sudo apt-get install gcc-<arch>-linux-gnu
```

This will install versions of ``cc``, ``ld``, ``objcopy``, and other commands that are required for cross-compiling and 
linking Novusk.

---

Make sure that there is a ``rust-toolchain`` file in the project's root directory, this should have "nightly" inside it
which sets the Rust compiler to nightly.

