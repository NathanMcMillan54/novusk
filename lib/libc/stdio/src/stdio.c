#include "stdio.h"

extern int printf(const char *fmt, ...) {
    ttprintf(fmt);
    return 0;
}
