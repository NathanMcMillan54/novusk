
// Disambiguation between system and our own definitions of the standard library require us to go back a few
// directories.
#include "../inc/stdlib.h"

int main() {
    if (!rust_ffi_test()) return 1;

    struct div_t divResult = {.quot = 2, .rem = 1};
    if (div(5, 2).quot != divResult.quot || div(5, 2).rem != divResult.rem) return 1;
}
