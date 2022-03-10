/* Copied and edited from https://github.com/rust-embedded/cortex-m-rt/blob/master/link.x.in */
INCLUDE src/boot/start/memory.x

ENTRY(_start)
EXTERN(__RESET_VECTOR);

SECTIONS
{
  PROVIDE(_stack_start = ORIGIN(RAM) + LENGTH(RAM));

  .vector_table ORIGIN(FLASH) :
  {
    LONG(_stack_start);

    KEEP(*(.vector_table.reset_vector));

    . = 0x0dc;
  } > FLASH

  PROVIDE(_stext = ADDR(.vector_table) + SIZEOF(.vector_table));

  .text _stext :
  {
    *(.text .text.*);
    . = ALIGN(4);
    __etext = .;
  } > FLASH

  .rodata __etext : ALIGN(4)
  {
    *(.rodata .rodata.*);

    . = ALIGN(4);
    __erodata = .;
  } > FLASH

  .data : AT(__erodata) ALIGN(4)
  {
    . = ALIGN(4);
    __sdata = .;
    *(.data .data.*);
    . = ALIGN(4);
    __edata = .;
  } > RAM

  __sidata = LOADADDR(.data);

  .bss : ALIGN(4)
  {
    . = ALIGN(4);
    __bss_start = .;
    *(.bss .bss.*);
    . = ALIGN(4);
    __bss_end = .;
  } > RAM

  .uninit (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    *(.uninit .uninit.*);
    . = ALIGN(4);
  } > RAM

  . = ALIGN(4);
  __sheap = .;

  /DISCARD/ :
  {
    *(.ARM.exidx);
    *(.ARM.exidx.*);
    *(.ARM.extab.*);
  }
}
