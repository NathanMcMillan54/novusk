## Rust

Function declaration:
```rust
fn function_name(arguments) {
    // The return type and starting curly brace should be on the same line as the function name
}
```

Importing with ``use``:
```rust
use alloc::vec::Vec; // uses should be at the top of the file and arranged in alphabetical order
use core::ptr::write_volatile;
use core::prelude; // Even though the libraries are in alphabetical order the modules aren't, this should be the second use
```

Importing with ``extern``:
```rust
// Importing external crates should also be in alphabetical order
#[macro_use] extern crate printk;
extern crate setup;
```

## TOML

Listing dependencies:
```toml
# Crate dependencies are listed first in alphabetecal order
[dependencies]
crate_1 = { path = "crate_1/" }
crate_a = "0.1.0"
crate_b = "0.1.0"

# Build dependencies come last
[build-dependencies]
build_crate = "0.1.0"
```

Feature names:
```toml
# Features should come last in the file and sorted alphabetcally
[features]
default = ["feature_b"]
feature_a = []
feature_b = []
```

## Assembly

Try to use GAS syntax inside the kernel as much as possible, this helps limit the tools needed for compiling since GCC
and AS should already be installed.

Global and external symbols:
```assembly
// These should be at the top of the file
.globl a_global_function
.extern an_external_function

/*
    code
*/

```
