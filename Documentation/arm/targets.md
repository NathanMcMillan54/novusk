### Novusk - ARM versions and targets

Last edited: 2022/11/5

---

These are targets that will be used for building depending on the value of ``ARCH``.

- ``aarch64`` - ``aarch64-novusk`` (Default)
- ``armv8-a`` - ``aarch64-novusk``
- ``arm`` - ``thumbv7em-none-eabihf`` (Default)
- ``armv7-a`` - ``arm-a-novusk``
- ``armv7-m`` - ``thumbv7m-none-eabi``
- ``armv7e-m`` - ``thumbv7em-none-eabihf``

Using ``aarch64`` (for arm64) will default to an ``armv8-a`` target and using ``arm`` (for arm32) will default to a 
``armv7e-m`` target.
