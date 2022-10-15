section .text

%include "kernel/kentry/limine.asm"

extern kmain
global _start

_start:
jmp kmain   ; main function linked from kernel src/main.rs

