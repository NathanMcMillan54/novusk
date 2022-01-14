### Building a single Package

Sometimes a library or important function can be needed outside of Novusk and so a single package needs to be compiled.

In the root of the project run this make command:
```commandline
make package CRATE=<package name> TARGET=<target>
```

The output file will be found in ``target/<package name>/release/lib<package name>.rlib``.
