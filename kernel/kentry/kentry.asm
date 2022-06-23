section .text

[extern kmain]
global _start
_start:
jmp kmain   ; main function linked from kernel.rs