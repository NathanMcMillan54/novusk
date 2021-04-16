.code16

#include "boot.h"

.extern init
.type init @function

.globl entry32
entry32:
    call goToProtectedMode
    call kentry

goToProtectedMode:
    cli
    CLEAR
    PROTECTED_MODE
    jmp .

.code32
kentry:
    call init
