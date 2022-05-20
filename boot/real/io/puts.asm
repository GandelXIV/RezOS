; string = (zero terminated char array)

; put string 
%macro real_puts 1
pusha       ; push all registers so they can be retrieved later
mov bx, %1  ; ld string pointer 
call real_puts_start
popa        ; retrieve registers
%endmacro

; put string on new line
%macro real_putsln 1
real_puts %1
scroll  ; defined in boot/real/io/nl.asm
%endmacro

real_puts_start:
mov ah, 0x0E  ; config

real_puts_loop:
mov al, [bx]  ; load new char

cmp al, 0     ; check for string null-termination
je real_puts_end

int INT_VIDEO       ; print current char
inc bx              ; select next char by incrementing the pointer
jmp real_puts_loop  ; repeat

real_puts_end:
ret
