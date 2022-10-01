ENTRY(_start)
OUTPUT_FORMAT(elf64-x86-64)

SECTIONS {
	.text : { *(.text) }
	.data : { *(.data) }
	.bss : { *(.bss) }
}