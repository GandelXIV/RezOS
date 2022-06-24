section .text

[extern kmain]

global _start
global _ZN4core9panicking5panic17h46e78c1781631473E ; temp fix

_start:
jmp kmain   ; main function linked from kernel.rs

_ZN4core9panicking5panic17h46e78c1781631473E:
jmp $