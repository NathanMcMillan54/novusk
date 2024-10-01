ENTRY(_start)

SECTIONS {
    . = 1M;

    .text :
    {
        *(.multiboot_header)
        *(.text)
    }
    
    .data :
    {
        . = ALIGN(4K);
    }

    .bss : 
    {
        . = ALIGN(4K);
    }
}