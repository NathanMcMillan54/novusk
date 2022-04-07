# Novusk - Kernel configurations

---

A Konfig file is a file used for determining what should be setup during the main kernel and after the kernel 
(your project). You should install the default konfig file and edit it for your usage.

Install:
```command line
novusk-dev-tools --install defualt_konfig.txt
```

The first thing you should do is change the ``AFTER_KERNEL`` option. By default it is ``Nothing`` and stops the kernel 
from setting up specific userspace stuff. Here are some things you can change it to:

    ``Kernel`` - Used as an extension to the kernel.
    ``Os`` - Used for setting up an operating. system. If you develop an OS supported by Novusk, you add the ``OS_NAME`` option.
    ``Server`` - Runs network tests then starts the userspace as a regular OS.
    ``BMApp`` - For bare metal apps.

Other options should be pretty self explanatory like kernel modules (read [this]() for more information about using your
own). More options and their uses can be found [here]().
