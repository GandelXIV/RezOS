ENTRY(_start)
OUTPUT_FORMAT(elf64-x86-64)

PHDRS
{
    text    	 PT_LOAD    FLAGS((1 << 0) | (1 << 2)) ; /* Execute + Read */
    rodata  	 PT_LOAD    FLAGS((1 << 2)) ;            /* Read only */
    data    	 PT_LOAD    FLAGS((1 << 1) | (1 << 2)) ; /* Write + Read */
}

SECTIONS {
	. = 0xFFFFFFFF80000000;

	.text : {
        *(.text .text.*)
    } :text
 
    /* Move to the next memory page for .rodata */
    . += CONSTANT(MAXPAGESIZE);
 
    .rodata : {
        *(.rodata .rodata.*)
    } :rodata
 
    /* Move to the next memory page for .data */
    . += CONSTANT(MAXPAGESIZE);
 
    .data : {
        *(.data .data.*)
    } :data
 
    .bss : {
        *(COMMON)
        *(.bss .bss.*)
    } :data
}