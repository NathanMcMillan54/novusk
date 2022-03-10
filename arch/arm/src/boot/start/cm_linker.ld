/* Copied and edited from https://github.com/rust-embedded/cortex-m-rt/blob/master/link.x.in */
INCLUDE src/boot/start/memory.x

EXTERN(__RESET_VECTOR);
EXTERN(_start);
ENTRY(_start);

EXTERN(__EXCEPTIONS);
EXTERN(DefaultHandler);
/*EXTERN(DefaultHandler_);*/

PROVIDE(NonMaskableInt = DefaultHandler);
EXTERN(HardFaultTrampoline);
PROVIDE(MemoryManagement = DefaultHandler);
PROVIDE(BusFault = DefaultHandler);
PROVIDE(UsageFault = DefaultHandler);
PROVIDE(SecureFault = DefaultHandler);
PROVIDE(SVCall = DefaultHandler);
PROVIDE(DebugMonitor = DefaultHandler);
PROVIDE(PendSV = DefaultHandler);
PROVIDE(SysTick = DefaultHandler);

/* PROVIDE(DefaultHandler = DefaultHandler_); */
PROVIDE(HardFault = HardFault_);

EXTERN(__INTERRUPTS);

PROVIDE(__pre_init = DefaultPreInit);

SECTIONS
{
  PROVIDE(_stack_start = ORIGIN(RAM) + LENGTH(RAM));

  .vector_table ORIGIN(FLASH) :
  {
    LONG(_stack_start);

    KEEP(*(.vector_table.reset_vector));
    __reset_vector = .;

    KEEP(*(.vector_table.exceptions));
    __eexceptions = .;

    KEEP(*(.vector_table.interrupts));
  } > FLASH

  PROVIDE(_stext = ADDR(.vector_table) + SIZEOF(.vector_table));

  /* ### .text */
  .text _stext :
  {
    __stext = .;
    *(.Reset);

    *(.text .text.*);

    *(.HardFaultTrampoline);
    *(.HardFault.*);

    . = ALIGN(4);
    __etext = .;
  } > FLASH

  .rodata : ALIGN(4)
  {
    . = ALIGN(4);
    __srodata = .;
    *(.rodata .rodata.*);

    . = ALIGN(4);
    __erodata = .;
  } > FLASH

  .data : ALIGN(4)
  {
    . = ALIGN(4);
    __sdata = .;
    *(.data .data.*);
    . = ALIGN(4);
  } > RAM AT>FLASH
  . = ALIGN(4);
  __edata = .;

  __sidata = LOADADDR(.data);

  .gnu.sgstubs : ALIGN(32)
  {
    . = ALIGN(32);
    __veneer_base = .;
    *(.gnu.sgstubs*)
    . = ALIGN(32);
    __veneer_limit = .;
  } > FLASH

  .bss (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    __bss_start = .;
    *(.bss .bss.*);
    *(COMMON);
    . = ALIGN(4);
  } > RAM
  . = ALIGN(4);
  __bss_end = .;

  .uninit (NOLOAD) : ALIGN(4)
  {
    . = ALIGN(4);
    __suninit = .;
    *(.uninit .uninit.*);
    . = ALIGN(4);
    __euninit = .;
  } > RAM

  PROVIDE(__sheap = __euninit);

  .got (NOLOAD) :
  {
    KEEP(*(.got .got.*));
  }

  /DISCARD/ :
  {
    *(.ARM.exidx);
    *(.ARM.exidx.*);
    *(.ARM.extab.*);
  }
}
