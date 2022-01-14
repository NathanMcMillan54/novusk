### Simple Building

#### Command:

The easiest way to compile Novusk is by running this command in the project's root directory:

```commandline
make all TARGET=<target> FEATURES=<features>
```

#### Arguments:

##### ``TARGET``:

The ``TARGET`` argument is for build target that is needed to compile Novusk, some of them can be found in ``targets/``.
Some build targets can't be found in ``targets/`` and have to be used from ``rustc``. The target you use is determined 
by the device you're compiling for, ``FEATURES`` talks about what target you should use.


##### ``FEATURES``:

The ``FEATURES`` argument is usually used for specifying the device or boot method Novusk will use, read 
``platform/<arch>.md`` for a list of devices and the targets needed for them.

