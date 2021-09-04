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
#[macro_use] extern crate printk; // #[macro_use] should be used on the same line so it's easier to read
extern crate setup;
```

## Assembly

Global and external symbols:
```assembly
// These should be at the top of the file
.globl a_global_function
.extern an_external_function

/*
    code
*/

```
