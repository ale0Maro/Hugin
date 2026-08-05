OUTPUT_ARCH(xtensa)

ENTRY(_start)

MEMORY
{
  iram0_0_seg (rx) : ORIGIN = 0x40080000, LENGTH = 0x20000
  dram0_0_seg (rw) : ORIGIN = 0x3FFB0000, LENGTH = 0x2C000
}

SECTIONS
{
  .text :
  {
    KEEP(*(.text._start))
    *(.text .text.*)
    *(.literal .literal.*)
  } > iram0_0_seg

  .rodata :
  {
    *(.rodata .rodata.*)
  } > dram0_0_seg

  .bss :
  {
    . = ALIGN(4);
    _sbss = .;
    *(.bss .bss.*)
    *(COMMON)
    . = ALIGN(4);
    _ebss = .;
  } > dram0_0_seg
}