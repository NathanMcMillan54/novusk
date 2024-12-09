.section .multiboot_header
header_start:
    .long 0xe85250d6
    .long 0
    .long header_end - header_start

    .long 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

    .short 0
    .short 0
    .long 8
header_end:
