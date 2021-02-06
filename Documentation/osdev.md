## OS development
This file talks about OS development with Novusk

---

In ``lib/src/user/`` there are functions and macros that can help with writing an OS with Novusk. With the ``lib/``
directory you can: print things, make very simple GUIs, and use a keyboard.

---

If you're writing an OS you'll need to use the ``drivers/`` directory and obviously the ``os/`` directory. 

When the userspace processes start, Novusk calls the function ``setup`` from ``drivers/src/os/setup.rs``, this calls
another setup function from a file or directory named after the argument given for ``OS`` when compiling. This will be
for early setup for your OS when the kernel is still doing things. Your main OS will be in ``os/``, there isn't much to 
say about this other than, "Have fun and make stuff!"