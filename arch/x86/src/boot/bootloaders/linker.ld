ENTRY(_start)
SECTIONS
{
    . = 1M;

    .text BLOCK(4K) : ALIGN(4K)
    {
    	*(.multiboot)
    	*(.text)
    }
}
