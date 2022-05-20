; string = (zero terminated char array)

; put string 
%macro rputs 1
pusha       ; push all registers so they can be retrieved later
mov bx, %1  ; ld string pointer 
call rputs_start
popa        ; retrieve registers
%endmacro

; put string on new line
%macro rputsln 1
rputs %1
rscroll  ; defined in boot/real/io/nl.asm
%endmacro

rputs_start:
mov ah, 0x0E  ; config

rputs_loop:
mov al, [bx]  ; load new char

cmp al, 0     ; check for string null-termination
je rputs_end

int INT_VIDEO       ; print current char
inc bx              ; select next char by incrementing the pointer
jmp rputs_loop  ; repeat

rputs_end:
ret
