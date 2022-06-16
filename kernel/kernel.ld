ENTRY(_start)
OUTPUT_FORMAT(elf64-x86-64)

SECTIONS {
	. = 0xFFFFFFFFFFE02000;
	.text : { *(.text) }
	.data : { *(.data) }
	.bss : { *(.bss) }
}