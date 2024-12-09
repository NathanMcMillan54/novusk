ENTRY(_start)

SECTIONS {
    . = 1M;

    .rodata :
    {
        KEEP(*(.multiboot_header))
        *(.rodata .rodata.*)
        . = ALIGN(4K);
    }

    .text :
    {
        *(.text .text.*)
        . = ALIGN(4K);
    }
    
    .data :
    {
        *(.data .data.*)
        . = ALIGN(4K);
    }

    .bss : 
    {
        *(.bss .bss.*)
        . = ALIGN(4K);
    }
}