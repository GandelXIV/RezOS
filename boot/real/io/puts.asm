INT_VIDEO equ 0x10

%macro real_puts 1
push bx
mov bx, %1
call real_puts_start
pop bx
%endmacro

%macro real_putsln 1
real_puts %1
scroll
%endmacro

real_puts_start:
pusha   ; push all registers so we can retrieve them after
mov ah, 0x0E

real_puts_loop:
mov al, [bx] ; load new char

cmp al, 0   ; check for string null-termination
je real_puts_end

int INT_VIDEO   ; print current char
inc bx  ; select next char
jmp real_puts_loop  ; repeat

real_puts_end:
popa    ; retrieve registers
ret
