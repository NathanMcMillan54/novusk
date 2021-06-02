# Project Structure

The usages of most of the directories in the Novusk project.

``arch/`` - Architectures

The ``arch/`` directory is used for the architecture specific kernels.

``drivers/`` - Drivers

The ``drivers/`` directory is used for software and some hardware drivers, any driver that isn't a: file system driver,
graphics card driver, or network driver, is put in this directory. This directory is also used for initializing main 
drivers.

``fs/`` - File systems

The ``fs/`` directory is used for support for file systems. If you are writing support for a new file system you would 
put you code in there.

``gpu/`` - Graphics cards and graphics drivers

The ``gpu/`` directory is used for all graphics drivers, if you're writing support for a specific graphics card or 
graphics mode.

``include/`` - Kernel functions

The ``include/`` directory is for functions used in the kernel.

``init`` - Main kernel

The ``init/`` directory initializes the kernel, it starts all the drivers and modules and sets up the userspace.

``kernel/`` - Main kernel functions and macros

The ``kernel/`` directory is used for main kernel functions and macros, like kernel printing and system calls.

``lib/`` - Libraries

The ``lib/`` directory is used for libraries used in the kernel.

``mm/``

The ``mm/`` directory is used for memory management functions and libraries.

``net/`` - Network drivers

The ``net/`` directory is used for network drivers. If you're writing a network driver you would put your code here.