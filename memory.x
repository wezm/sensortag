/* Linker script for the STM32F103C8T6 */
/*
#define FLASH_BASE              0x0
#define FLASH_SIZE              0x20000
#define RAM_BASE                0x20000000
#define RAM_SIZE                0x5000
*/
MEMORY
{
  FLASH (RX) : ORIGIN = 0x00000000, LENGTH = 128K
  RAM (RWX) : ORIGIN = 0x20000000, LENGTH = 20K
}

/*

SECTIONS
{
    .text           :   > FLASH
    .const          :   > FLASH
    .constdata      :   > FLASH
    .rodata         :   > FLASH
    .cinit          :   > FLASH
    .pinit          :   > FLASH
    .init_array     :   > FLASH
    .emb_text       :   > FLASH
    .ccfg           :   > FLASH (HIGH)

#ifdef __TI_COMPILER_VERSION__
#if __TI_COMPILER_VERSION__ >= 15009000
    .TI.ramfunc     : {} load=FLASH, run=SRAM, table(BINIT)
#endif
#endif
    .data           :   > SRAM
    .bss            :   > SRAM
    .sysmem         :   > SRAM
    .stack          :   > SRAM (HIGH)
    .nonretenvar    :   > SRAM
}
*/
