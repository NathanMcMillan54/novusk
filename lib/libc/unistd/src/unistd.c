#include "unistd.h"

extern int add_syscalls();

int initialize_unistd() {
    int ret = add_syscalls();

    return ret;
}
