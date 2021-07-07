// Aarch64 RaspberryPi 3 - 4 startup file
// TODO: In the future when documentation is being written and it talks about project structer, explain why Aarch64
// files are in the arm crate

.globl _start
.type _start, function
.size _start, . - _start

_start:
    bl die

die:
    wfe
    b die
