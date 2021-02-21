.extern kernel_time
.extern time_init

.globl currentTime
.globl startTime

currentTime:
    bl kernelTime

startTime:
    bl time_init
