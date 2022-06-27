section .text

[extern kmain]

global _start
global _ZN4core9panicking5panic17hf7222a04f4380515E ; temp fix

_start:
jmp kmain   ; main function linked from kernel.rs

_ZN4core9panicking5panic17hf7222a04f4380515E:
jmp $